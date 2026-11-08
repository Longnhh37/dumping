/*
Draft ──submit──▶ PendingApproval ──approve(&ManagerToken)──▶ Approved ──execute(ApprovalReceipt)──▶ Executing ──settle──▶ Settled
  ▲                    │   │                                      (receipt bị consume)
  └──────withdraw──────┘   └──reject──▶ Cancelled
*/

// put everything in a private module so nothing can be forge outside
mod wire {
    use std::marker::PhantomData;

    // --- States (zero-sized markers) ---
    pub struct Draft;
    pub struct PendingApproval;
    pub struct Approved;
    pub struct Executing;
    pub struct Settled;
    pub struct Cancelled;

    // --- Capability token: only mangers can approve ---
    // No Clone/Copy (move semantics), and the private field blocks outside construction.
    #[must_use]
    pub struct ManagerToken {
        _private: (),
    }

    /// Stand-in for a real auth system: the only way to obtain a token.
    pub fn authenticate_manager(name: &str, password: &str) -> Option<ManagerToken> {
        (name == "alice" && password == "secret").then_some(ManagerToken { _private: () })
    }

    // --- Single-use proof that the transfer was approved ---
    // Deliberately NOT Clone/Copy, so `execute()` can only be called once per receipt
    #[must_use = "an approval receipt must be passed to execute()"]
    pub struct ApprovalReceipt {
        _private: (),
        pub transfer_id: u64,
        pub amount_cents: u64,
    }

    pub struct Transfer<S> {
        id: u64,
        to: String,
        amount_cents: u64,
        _state: PhantomData<S>,
    }

    // Shared helper: change the state marker, keep all data.
    // Private, so only this mod can move a Transfer between states.
    impl<S> Transfer<S> {
        fn into_state<N>(self) -> Transfer<N> {
            Transfer {
                id: self.id,
                to: self.to,
                amount_cents: self.amount_cents,
                _state: PhantomData,
            }
        }
    }

    impl Transfer<Draft> {
        pub fn new(id: u64, to: impl Into<String>, amount_cents: u64) -> Self {
            Self {
                id,
                to: to.into(),
                amount_cents,
                _state: PhantomData,
            }
        }

        pub fn submit(self) -> Transfer<PendingApproval> {
            println!("#{}: submitted for approval", self.id);
            self.into_state()
        }
    }

    impl Transfer<PendingApproval> {
        /// Approve requires a manager token and PRODUCES a single-use receipt.
        pub fn approve(self, _msg: &ManagerToken) -> (Transfer<Approved>, ApprovalReceipt) {
            println!("#{}: approved", self.id);
            let receipt = ApprovalReceipt {
                _private: (),
                transfer_id: self.id,
                amount_cents: self.amount_cents,
            };

            (self.into_state(), receipt)
        }

        /// Rejection is terminal
        pub fn reject(self) -> Transfer<Cancelled> {
            println!("#{}: rejected", self.id);
            self.into_state()
        }

        /// Safe branch back: the sender takes the draft back to edit it.
        pub fn withdraw(self) -> Transfer<Draft> {
            println!("#{}: withdrawn back to draft", self.id);
            self.into_state()
        }
    }

    impl Transfer<Approved> {
        /// Consume the receipt (moved by value), so a second execute is impossible.
        pub fn execute(self, receipt: ApprovalReceipt) -> Transfer<Executing> {
            println!(
                "#{}: sending {} cents to {} (receipt for #{})",
                self.id, receipt.amount_cents, self.to, receipt.transfer_id
            );
            self.into_state()
        }
    }

    impl Transfer<Executing> {
        pub fn settle(self) -> Transfer<Settled> {
            println!("#{}: settled", self.id);
            self.into_state()
        }
    }

    impl Transfer<Settled> {
        pub fn summary(&self) -> String {
            format!(
                "#{} -> {}: {} cents (done)",
                self.id, self.to, self.amount_cents
            )
        }
    }
}

use wire::{Transfer, authenticate_manager};

fn main() {
    let draft = Transfer::new(1001, "Bob", 250_00);

    // draft.execute(...);
    //       ^^^^^^^ ERROR: no method `execute` for `Transfer<Draft>`

    let pending = draft.submit();

    let Some(mgr) = authenticate_manager("alice", "secret") else {
        eprintln!("not a manager");
        return;
    };

    // Bug class 1 (wrong order) is blocked at compile time.
    // Bug class 2 (no privilege) is blocked: approve() needs &ManagerToken.
    let (approved, receipt) = pending.approve(&mgr);

    let executing = approved.execute(receipt);
    // Bug class 3 (double execute) is blocked
    // approved.execute(receipt);
    //                  ^^^^^^^ ERROR: use of moved value: `approved` / `receipt`

    let settled = executing.settle();
    println!("{}", settled.summary());

    // Forging proofs does not compile either:
    // let fake = wire::ApprovalReceipt { _private: (), transfer_id: 1, amount_cents: 1 };
    //                                    ^^^^^^^^ ERROR: Field `_private` is private
}
