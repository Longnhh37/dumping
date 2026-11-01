/*
App implements:  HasClock, HasLogger, HasStore      (you write these 3 impls, ~5 lines each)
                      │         │          │
                      ▼         ▼          ▼
blanket impl:         AuditMixin  CacheMixin        (auto, because ingredients are satisfied)
                           └────┬────┘
                                ▼
blanket impl:               SessionMixin            (auto, because both mixins are satisfied)
*/

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

// ===============================================================================
// Layer 0: low-level capabilities (what the outside world provides)
// ===============================================================================
trait Clock {
    fn now(&self) -> u64;
}
trait Logger {
    fn log(&self, msg: &str);
}
trait KvStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, val: String);
}

// ===============================================================================
// Layer 1: ingredients. "I have capabilities X", exposed via an associated type
// ===============================================================================
trait HasClock {
    type C: Clock;
    fn clock(&self) -> &Self::C;
}
trait HasLogger {
    type L: Logger;
    fn logger(&self) -> &Self::L;
}
trait HasStore {
    type S: KvStore;
    fn store(&self) -> &Self::S;
}

// ===============================================================================
// Layer 2: mixins. Required ingredients are supertraits, logic is default methods
// ===============================================================================

// Needs: clock + logger
trait AuditMixin: HasClock + HasLogger {
    fn audit(&self, action: &str) {
        let t = self.clock().now();
        self.logger().log(&format!("[t={t}] {action}"));
    }
}

// Needs: store + clock
trait CacheMixin: HasStore + HasClock {
    fn cache_set(&self, key: &str, val: &str) {
        // store "timestamp|value" so we can check expiry later
        let entry = format!("{}|{val}", self.clock().now());
        self.store().set(key, entry);
    }

    fn cache_get(&self, key: &str, ttl: u64) -> Option<String> {
        let entry = self.store().get(key)?;
        let (ts, val) = entry.split_once('|')?;
        let ts: u64 = ts.parse().ok()?;
        // expired entries are treated as missing
        (self.clock().now() - ts <= ttl).then(|| val.to_string())
    }
}

// mixin built on top of OTHER mixins. It needs nothing directly:
// AuditMixin + CacheMixin already pull in clock, logger, and store
trait SessionMixin: AuditMixin + CacheMixin {
    fn login(&self, user: &str) -> String {
        let token = format!("tok-{user}");
        self.cache_set(&format!("session:{user}"), &token); // from CacheMixin
        self.audit(&format!("login {user}")); // from AuditMixin
        token
    }

    fn is_logged_in(&self, user: &str, ttl: u64) -> bool {
        self.cache_get(&format!("session:{user}"), ttl).is_some()
    }
}

// ============================================================================
// Layer 3: blanket impls. "Provide the ingredients, get the methods."
// ============================================================================
impl<T: HasClock + HasLogger> AuditMixin for T {}
impl<T: HasStore + HasClock> CacheMixin for T {}
impl<T: AuditMixin + CacheMixin> SessionMixin for T {}

// ============================================================================
// Concrete implementations (in-memory, so we can run without any infra)
// ============================================================================
struct FakeClock(Cell<u64>);
impl Clock for FakeClock {
    fn now(&self) -> u64 {
        self.0.get()
    }
}

struct MemLogger(RefCell<Vec<String>>);
impl Logger for MemLogger {
    fn log(&self, msg: &str) {
        self.0.borrow_mut().push(msg.to_string());
    }
}

struct MemStore(RefCell<HashMap<String, String>>);
impl KvStore for MemStore {
    fn get(&self, key: &str) -> Option<String> {
        self.0.borrow().get(key).cloned()
    }
    fn set(&self, key: &str, val: String) {
        self.0.borrow_mut().insert(key.to_string(), val);
    }
}

// ----------------------------------------------------------------------------
// Full app: has all three ingredients -> gets ALL mixins
// ----------------------------------------------------------------------------
struct App {
    clock: FakeClock,
    logger: MemLogger,
    store: MemStore,
}

impl HasClock for App {
    type C = FakeClock;
    fn clock(&self) -> &Self::C {
        &self.clock
    }
}
impl HasLogger for App {
    type L = MemLogger;
    fn logger(&self) -> &Self::L {
        &self.logger
    }
}
impl HasStore for App {
    type S = MemStore;
    fn store(&self) -> &Self::S {
        &self.store
    }
}

// ----------------------------------------------------------------------------
// Partial app: NO store -> gets AuditMixin only
// ----------------------------------------------------------------------------
struct LogOnly {
    clock: FakeClock,
    logger: MemLogger,
}

impl HasClock for LogOnly {
    type C = FakeClock;
    fn clock(&self) -> &Self::C {
        &self.clock
    }
}
impl HasLogger for LogOnly {
    type L = MemLogger;
    fn logger(&self) -> &Self::L {
        &self.logger
    }
}

fn example() {
    // --- Full app: everything works ---
    let app = App {
        clock: FakeClock(Cell::new(100)),
        logger: MemLogger(RefCell::new(vec![])),
        store: MemStore(RefCell::new(HashMap::new())),
    };

    app.login("bob"); // SessionMixin -> CacheMixin + AuditMixin
    println!("t=100 logged in? {}", app.is_logged_in("bob", 5)); // true

    app.clock.0.set(110); // advance time by 10
    println!("t=110, ttl=5  -> {}", app.is_logged_in("bob", 5)); // false (expired)
    println!("t=110, ttl=60 -> {}", app.is_logged_in("bob", 60)); // true

    println!("logs: {:?}", app.logger.0.borrow());

    // --- Partial app: only the mixins whose ingredients it has ---
    let lo = LogOnly {
        clock: FakeClock(Cell::new(7)),
        logger: MemLogger(RefCell::new(vec![])),
    };
    lo.audit("boot"); // OK: has HasClock + HasLogger
    println!("logs: {:?}", lo.logger.0.borrow());

    // lo.cache_set("k", "v");  // Compile error: `LogOnly: HasStore` is not satisfied
    // lo.login("bob");        // Compile error: SessionMixin needs CacheMixin -> HasStore
}
