use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

use crate::{constants, helpers::spawn_app};

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    // assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    // arrange
    let app = spawn_app().await;
    let body = constants::FULL_BODY;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    // act
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // arrange
    let app = spawn_app().await;
    let body = constants::FULL_BODY;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    // act
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to fetch saved subscriptions");

    assert_eq!(saved.email, constants::EMAIL);
    assert_eq!(saved.name, constants::NAME);
    assert_eq!(saved.status, constants::STATUS_CONFIRM);
}
