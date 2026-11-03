use std::cell::RefCell;
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};

pub struct Bank {
    balances: RefCell<HashMap<String, i64>>,
}

pub struct Tx<'a> {
    bank: &'a Bank,
}

impl Tx<'_> {
    pub fn balance(&self, name: &str) -> i64 {
        self.bank.balances.borrow().get(name).copied().unwrap_or(0)
    }

    pub fn deposit(&mut self, name: &str, amount: i64) {
        println!("\tdeposit {amount} -> {name}");
        *self
            .bank
            .balances
            .borrow_mut()
            .entry(name.to_owned())
            .or_insert(0) += amount;
    }

    pub fn withdraw(&mut self, name: &str, amount: i64) -> Result<(), String> {
        let available = self.balance(name);
        if available < amount {
            return Err(format!("{name} has {available}, needs {amount}"));
        }
        println!("\twithdraw {amount} <- {name}");
        self.deposit(name, -amount);
        Ok(())
    }
}

struct RollbackGuard<'a> {
    bank: &'a Bank,
    snapshot: Option<HashMap<String, i64>>,
}

impl Drop for RollbackGuard<'_> {
    fn drop(&mut self) {
        if let Some(snapshot) = self.snapshot.take() {
            println!("\t[db] rollback");
            *self.bank.balances.borrow_mut() = snapshot;
        }
    }
}

impl Bank {
    pub fn new<const N: usize>(initial: [(&str, i64); N]) -> Self {
        let map = initial.map(|(k, v)| (k.to_owned(), v));
        Bank {
            balances: RefCell::new(HashMap::from(map)),
        }
    }

    pub fn balance(&self, name: &str) -> i64 {
        self.balances.borrow().get(name).copied().unwrap_or(0)
    }

    pub fn transaction<R, E>(&self, f: impl FnOnce(&mut Tx<'_>) -> Result<R, E>) -> Result<R, E> {
        println!("\t[db] begin");
        let mut guard = RollbackGuard {
            bank: self,
            snapshot: Some(self.balances.borrow().clone()),
        };
        let mut tx = Tx { bank: self };

        let result = f(&mut tx);

        if result.is_ok() {
            // disarm guard: nothing to rollback
            guard.snapshot = None;
            println!("\t[db] commit");
        }
        // Err: 'guard' is dropped here and restores the snapshot
        result
    }
}

fn main() {
    let bank = Bank::new([("alice", 100), ("bob", 50)]);
    println!("Initial balance: {:?}\n", bank.balances.borrow());

    // 1. Success path -> commit
    println!("transfer 30 alice -> bob");
    let r = bank.transaction(|tx| {
        tx.withdraw("alice", 30)?;
        tx.deposit("bob", 30);
        Ok::<_, String>(())
    });
    println!(
        "\tresult = {r:?}, alice = {}, bob = {}\n",
        bank.balance("alice"),
        bank.balance("bob")
    );

    // 2. Fails halfway via `?` -> bob's deposit is rolled back
    println!("deposit to bob, then overdraw alice");
    let r = bank.transaction(|tx| {
        tx.deposit("bob", 1_000);
        tx.withdraw("alice", 500)?; // Err -> early return
        Ok::<_, String>(())
    });
    println!(
        "\tresult = {r:?}, alice = {}, bob = {}\n",
        bank.balance("alice"),
        bank.balance("bob")
    );

    // 3. Panic inside the closure -> guard still rolls back
    println!("panic inside transaction");
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let _ = bank.transaction(|tx| -> Result<(), String> {
            tx.deposit("bob", 9_999);
            panic!("boom");
        });
    }));
    println!(
        "\talice = {}, bob = {}",
        bank.balance("alice"),
        bank.balance("bob")
    );

    // The handle cannot escape the closure:
    // let escaped = bank.transaction(|tx| Ok::<_, String>(tx));
    // ERROR: lifetime error, `Tx` cannot be returned out of the closure
}
