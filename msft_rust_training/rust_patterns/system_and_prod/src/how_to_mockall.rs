use mockall::automock;

#[automock] // generates `MockEmailSender2`
trait EmailSender2 {
    fn send(&self, to: &str, body: &str) -> Result<(), String>;
}

struct UserService2<S: EmailSender2> {
    sender: S,
}

impl<S: EmailSender2> UserService2<S> {
    fn register(&self, name: &str, email: &str) -> Result<(), String> {
        self.sender.send(email, &format!("Welcome, {name}!"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_sends_exactly_one_email() {
        let mut mock = MockEmailSender2::new();

        // Expectation: `send` must be called exactly once with these args
        mock.expect_send()
            .times(1)
            .withf(|to, body| to == "bob@example.com" && body == "Welcome, bob!")
            .returning(|_, _| Ok(()));

        let service = UserService2 { sender: mock };
        service.register("bob", "bob@example.com").unwrap();
        // On drop, mockall verifies `times(1)` and fails the test if violated
    }
}
