// idea: UserService doesn't call SMTP but through 'EmailSender' trait.
// test: input a Mock as argument, and record call-site

// 1. Define the behavior as a trait
trait EmailSender {
    fn send(&self, to: &str, body: &str) -> Result<(), String>;
}

// 2. The service depends on the trait, not on a concrete type
struct UserService<S: EmailSender> {
    sender: S,
}

impl<S: EmailSender> UserService<S> {
    fn new(sender: S) -> Self {
        Self { sender }
    }

    fn register(&self, name: &str, email: &str) -> Result<(), String> {
        if !email.contains('@') {
            return Err(format!("invalid email: {email}"));
        }
        self.sender.send(email, &format!("Welcome, {name}!"))
    }
}

// 3. production implementation
struct SmtpSender;

impl EmailSender for SmtpSender {
    fn send(&self, to: &str, body: &str) -> Result<(), String> {
        println!("sending to {to}: {body}");
        Ok(())
    }
}

fn main_eg() {
    let service = UserService::new(SmtpSender);
    service.register("Alice", "alice@example.com").unwrap();
}

// 4. tests use a mock instead of the real sender
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // Mock: records every call, and can be told to fail
    // RefCell is needed because 'send' takes '&self'
    #[derive(Default)]
    struct MockSender {
        sent: RefCell<Vec<(String, String)>>,
        should_fail: bool,
    }

    impl EmailSender for MockSender {
        fn send(&self, to: &str, body: &str) -> Result<(), String> {
            if self.should_fail {
                return Err("smtp down".into());
            }
            self.sent.borrow_mut().push((to.into(), body.into()));
            Ok(())
        }
    }

    #[test]
    fn register_sends_welcome_email() {
        let service = UserService::new(MockSender::default());

        service.register("Alice", "alice@example.com").unwrap();

        let sent = service.sender.sent.borrow();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].0, "alice@example.com");
        assert_eq!(sent[0].1, "Welcome, Alice!");
    }

    #[test]
    fn invalid_email_sends_nothing() {
        let service = UserService::new(MockSender::default());

        assert!(service.register("Alice", "not-an-email").is_err());
        assert!(service.sender.sent.borrow().is_empty());
    }

    #[test]
    fn sender_error_is_propagated() {
        let mock = MockSender {
            should_fail: true,
            ..Default::default()
        };
        let service = UserService::new(mock);

        assert_eq!(
            service.register("Alice", "alice@example.com"),
            Err("smtp down".into())
        );
    }
}
