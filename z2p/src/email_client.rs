use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        Self {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);

        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject,
            html_body: html_content,
            text_body: text_content,
        };

        self.http_client
            .post(&url)
            .header(
                "X-Resend-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

#[cfg(test)]
mod tests {
    // ==========================================================
    // Test helper + setup
    // ==========================================================
    use claim::{assert_err, assert_ok};
    use fake::{
        Fake, Faker,
        faker::internet::en::SafeEmail,
        faker::lorem::en::{Paragraph, Sentence},
    };
    use secrecy::Secret;
    use wiremock::{
        Mock, MockServer, Request, ResponseTemplate,
        matchers::{any, header, header_exists, method, path},
    };

    use crate::{domain::SubscriberEmail, email_client::EmailClient};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let res: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);

            if let Ok(body) = res {
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                false
            }
        }
    }

    struct SendEmailTestContext {
        mock_server: MockServer,
        email_client: EmailClient,
        subscriber_email: SubscriberEmail,
        subject: String,
        content: String,
    }

    async fn send_email_test_setup() -> SendEmailTestContext {
        let mock_server = MockServer::start().await;

        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(
            mock_server.uri(),
            sender,
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200),
        );

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        SendEmailTestContext {
            mock_server,
            email_client,
            subscriber_email,
            subject,
            content,
        }
    }

    // ==========================================================
    // Test
    // ==========================================================

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        // arrange
        let setup = send_email_test_setup().await;

        Mock::given(header_exists("X-Resend-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&setup.mock_server)
            .await;

        // act
        let _ = setup
            .email_client
            .send_email(
                &setup.subscriber_email,
                &setup.subject,
                &setup.content,
                &setup.content,
            )
            .await;

        // assert
    }

    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        // arrange
        let setup = send_email_test_setup().await;

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&setup.mock_server)
            .await;

        // act
        let outcome = setup
            .email_client
            .send_email(
                &setup.subscriber_email,
                &setup.subject,
                &setup.content,
                &setup.content,
            )
            .await;

        // assert
        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        // arrange
        let setup = send_email_test_setup().await;

        // assert
        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&setup.mock_server)
            .await;

        // act
        let outcome = setup
            .email_client
            .send_email(
                &setup.subscriber_email,
                &setup.subject,
                &setup.content,
                &setup.content,
            )
            .await;

        // assert
        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        // arrange
        let setup = send_email_test_setup().await;

        let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));

        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&setup.mock_server)
            .await;

        // act
        let outcome = setup
            .email_client
            .send_email(
                &setup.subscriber_email,
                &setup.subject,
                &setup.content,
                &setup.content,
            )
            .await;

        // assert
        assert_err!(outcome);
    }
}
