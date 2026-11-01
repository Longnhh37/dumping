use std::fmt;

// Display is a supertrait of Error
trait Error: fmt::Display + fmt::Debug {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
// Any type implementing Error MUST also implement Display and Debug

// Build your own hierarchies:
trait Identifiable {
    fn id(&self) -> u64;
}

trait Timestamped {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
}

// Entity requires both:
trait Entity: Identifiable + Timestamped {
    fn is_active(&self) -> bool;
}

// Implementing Entity forces you to implement all three:
struct User {
    id: u64,
    name: String,
    created: chrono::DateTime<chrono::Utc>,
}

impl Identifiable for User {
    fn id(&self) -> u64 {
        self.id
    }
}
impl Timestamped for User {
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created
    }
}
impl Entity for User {
    fn is_active(&self) -> bool {
        true
    }
}
