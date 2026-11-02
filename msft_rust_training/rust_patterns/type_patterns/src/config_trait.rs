// Config trait
// Combines dependent types (store, cache, mailer,...) into associated types of a single trait
// struct needs only ONE parameter Cfg, other types are inferred through Cfg::Store, Cfg::Cache, etc

use std::collections::HashMap;
use std::io::Empty;
use std::sync::Mutex;

// ===============================================================================
// trait setup
// ===============================================================================
#[derive(Debug)]
enum AppError {
    NotFound,
    Storage(String),
}

// Component traits
trait UserStore {
    fn find(&self, id: u64) -> Result<Option<String>, AppError>;
    fn save(&self, id: u64, name: &str) -> Result<(), AppError>;
}

trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn put(&self, key: &str, value: String);
}

trait Mailer {
    fn send(&self, to: &str, body: &str) -> Result<(), AppError>;
}

// Config trait: one associated type per component
trait AppConfig {
    type Store: UserStore;
    type Cache: Cache;
    type Mailer: Mailer;
}

// Exactly ONE generic parameter, no matter how many components
struct UserService<Cfg: AppConfig> {
    store: Cfg::Store,
    cache: Cfg::Cache,
    mailer: Cfg::Mailer,
}

impl<Cfg: AppConfig> UserService<Cfg> {
    fn new(store: Cfg::Store, cache: Cfg::Cache, mailer: Cfg::Mailer) -> Self {
        Self {
            store,
            cache,
            mailer,
        }
    }

    fn get_name(&self, id: u64) -> Result<String, AppError> {
        let key = format!("user:{id}");
        if let Some(hit) = self.cache.get(&key) {
            return Ok(hit);
        }
        let name = self.store.find(id)?.ok_or(AppError::NotFound)?;
        self.cache.put(&key, name.clone());
        Ok(name)
    }

    fn register(&self, id: u64, name: &str, email: &str) -> Result<(), AppError> {
        self.store.save(id, name)?;
        self.mailer.send(email, &format!("Welcome, {name}"))
    }
}

// Downstream code also needs only ONE generic param
fn handle_signup<Cfg: AppConfig>(svc: &UserService<Cfg>, email: &str) -> Result<(), AppError> {
    svc.register(42, "alice", email)
}

// ===============================================================================
// Wiring
// ===============================================================================
struct PostgresStore;
struct RedisCache;
struct SmtpMailer;

impl UserStore for PostgresStore {
    fn find(&self, _id: u64) -> Result<Option<String>, AppError> {
        Ok(Some("alice".into()))
    }
    fn save(&self, _id: u64, _name: &str) -> Result<(), AppError> {
        Ok(())
    }
}
impl Cache for RedisCache {
    fn get(&self, _key: &str) -> Option<String> {
        None
    }
    fn put(&self, _key: &str, _value: String) {}
}
impl Mailer for SmtpMailer {
    fn send(&self, _to: &str, _body: &str) -> Result<(), AppError> {
        Ok(())
    }
}

struct ProdApp;
impl AppConfig for ProdApp {
    type Store = PostgresStore;
    type Cache = RedisCache;
    type Mailer = SmtpMailer;
}

fn example() {
    let svc = UserService::<ProdApp>::new(PostgresStore, RedisCache, SmtpMailer);
    handle_signup(&svc, "alice@example.com").unwrap();
}

// ===============================================================================
// Wiring test
// ===============================================================================
#[derive(Default)]
struct MemStore(Mutex<HashMap<u64, String>>);
#[derive(Default)]
struct MemCache(Mutex<HashMap<String, String>>);
#[derive(Default)]
struct SpyMailer {
    sent: Mutex<Vec<(String, String)>>,
}

impl UserStore for MemStore {
    fn find(&self, id: u64) -> Result<Option<String>, AppError> {
        Ok(self.0.lock().unwrap().get(&id).cloned())
    }
    fn save(&self, id: u64, name: &str) -> Result<(), AppError> {
        self.0.lock().unwrap().insert(id, name.to_owned());
        Ok(())
    }
}
impl Cache for MemCache {
    fn get(&self, key: &str) -> Option<String> {
        self.0.lock().unwrap().get(key).cloned()
    }
    fn put(&self, key: &str, value: String) {
        self.0.lock().unwrap().insert(key.to_owned(), value);
    }
}
impl Mailer for SpyMailer {
    fn send(&self, to: &str, body: &str) -> Result<(), AppError> {
        self.sent
            .lock()
            .unwrap()
            .push((to.to_owned(), body.to_owned()));
        Ok(())
    }
}

struct TestApp;
impl AppConfig for TestApp {
    type Store = MemStore;
    type Cache = MemCache;
    type Mailer = SpyMailer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_then_read_and_mail_sent() {
        let svc = UserService::<TestApp>::new(
            MemStore::default(),
            MemCache::default(),
            SpyMailer::default(),
        );
        handle_signup(&svc, "alice@example.com").unwrap();

        assert_eq!(svc.get_name(42).unwrap(), "alice");
        assert_eq!(svc.mailer.sent.lock().unwrap().len(), 1);
    }
}
