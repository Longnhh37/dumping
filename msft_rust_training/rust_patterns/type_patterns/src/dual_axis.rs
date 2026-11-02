// dual-axis pattern = Handle<Vendor, State>
// axis 1: vender, pick with trait bound
// axis 2: statem typestatem pick with marker type
// each impl represents one cell in matrix (vendor x state)

// Domain: payment gateway
// Flow: authorize -> capture -> refund
// Basic gateway        authorize()             capture(), cancel()                 refund()
// Partial-capable      authorize()             capture(), capture_partial(),       refund(),
// gateway                                      cancel()                            refund_partial()

/*
                       STATE axis (S)  ->
              +-------------+------------------+-------------------------+-----------+----------+
              |   Pending   |    Authorized    |          Captured       | Cancelled | Refunded |
+-------------+-------------+------------------+-------------------------+-----------+----------+
| BasicBank   | authorize() | capture()        | refund()                |     -     |    -     |
| (Gateway)   | cancel()    | cancel()         |                         | (terminal)|(terminal)|
|             |             |                  |                         |           |          |
+-------------+-------------+------------------+-------------------------+-----------+----------+
| StripeLike  | authorize() | capture()        | refund()                |     -     |    -     |
| (Gateway +  | cancel()    | capture_partial()| refund_partial()        | (terminal)|(terminal)|
| PartialGW)  |             | cancel()         | (&mut, stays Capture)   |           |          |
+-------------+-------------+------------------+-------------------------+-----------+----------+
  ^
  | VENDOR axis (G)

Which impl block fills which cell:

  impl<G: Gateway>                       Payment<G, Pending>      -> authorize
  impl<G: Gateway>                       Payment<G, Authorized>   -> capture
  impl<G: PartialGateway>                Payment<G, Authorized>   -> capture_partial   (Stripe row only)
  impl<G: Gateway, S: Cancellable>       Payment<G, S>            -> cancel            (Pending + Authorized columns)
  impl<G: Gateway, S: Refundable>        Payment<G, S>            -> refund            (Captured column)
  impl<G: PartialGateway, S: Refundable> Payment<G, S>            -> refund_partial    (Stripe row, Captured column)

State transitions (same for both vendors):

  Pending --authorize--> Authorized --capture--> Captured --refund--> Refunded
     |                       |
     +------- cancel --------+--------> Cancelled


*/

// What the compiler prevents (each snippet below fails to compile):
//
// 1. Payment::new(BasicBank, 100).authorize().capture_partial(50)
//    error: no method `capture_partial`
//    reason: BasicBank does not impl PartialGateway
//
// 2. Payment::new(StripeLike, 100).capture()
//    error: no method `capture`
//    reason: Pending has no `capture`, must `authorize` first
//
// 3. Payment::new(StripeLike, 100).authorize().refund()
//    error: no method `refund`
//    reason: Authorized does not impl Refundable
//
// 4. p.capture().cancel()
//    error: no method `cancel`
//    reason: Captured does not impl Cancellable
//
// 5. let a = p.authorize(); let b = p.authorize();
//    error: use of moved value
//    reason: `authorize(self)` consumes `self`

use std::marker::PhantomData;

struct Pending;
struct Authorized;
struct Captured;
struct Cancelled;
struct Refunded;

// Marker traits: which states grant which capability
trait Cancellable {}
impl Cancellable for Pending {}
impl Cancellable for Authorized {}

trait Refundable {}
impl Refundable for Captured {}

// Vendor axis: every gateway implements the base trait...
trait Gateway {
    fn raw_authorize(&mut self, cents: u64);
    fn raw_capture(&mut self, cents: u64);
    fn raw_cancel(&mut self);
    fn raw_refund(&mut self, cents: u64);
}
// ...and only some gateways implement the extended super-trait
trait PartialGateway: Gateway {
    fn raw_capture_partial(&mut self, cents: u64);
    fn raw_refund_partial(&mut self, cents: u64);
}

#[derive(Debug)]
struct OverRefund;

struct Payment<G, S = Pending> {
    gateway: G,
    amount_cents: u64,
    _state: PhantomData<S>,
}

// shared helper: change only the state type, move everything else
impl<G, S> Payment<G, S> {
    fn transition<T>(self) -> Payment<G, T> {
        Payment {
            gateway: self.gateway,
            amount_cents: self.amount_cents,
            _state: PhantomData,
        }
    }
}

// row 1: construction and authorize, any gateway
impl<G: Gateway> Payment<G, Pending> {
    fn new(gateway: G, amount_cents: u64) -> Self {
        Payment {
            gateway,
            amount_cents,
            _state: PhantomData,
        }
    }

    fn authorize(mut self) -> Payment<G, Authorized> {
        self.gateway.raw_authorize(self.amount_cents);
        self.transition()
    }
}

// Authorized + any gateway: full capture
impl<G: Gateway> Payment<G, Authorized> {
    fn capture(mut self) -> Payment<G, Captured> {
        self.gateway.raw_capture(self.amount_cents);
        self.transition()
    }
}

// Authorized + partial-capable gateway only
impl<G: PartialGateway> Payment<G, Authorized> {
    fn capture_partial(mut self, cents: u64) -> Payment<G, Captured> {
        self.gateway.raw_capture_partial(cents);
        self.amount_cents = cents;
        self.transition()
    }
}

// Any gateway, any state marked Cancellable (Pending or Authorized)
impl<G: Gateway, S: Cancellable> Payment<G, S> {
    fn cancel(mut self) -> Payment<G, Cancelled> {
        self.gateway.raw_cancel();
        self.transition()
    }
}

// Any gateway, any state marked Refundable (Captured)
impl<G: Gateway, S: Refundable> Payment<G, S> {
    fn refund(mut self) -> Payment<G, Refunded> {
        self.gateway.raw_refund(self.amount_cents);
        self.transition()
    }
}

// Partial-capable gateway + Refundable state
impl<G: PartialGateway, S: Refundable> Payment<G, S> {
    // &mut self: partial refund keeps the payment in Captured
    fn refund_partial(&mut self, cents: u64) -> Result<(), OverRefund> {
        if cents > self.amount_cents {
            return Err(OverRefund);
        }
        self.gateway.raw_refund_partial(cents);
        self.amount_cents -= cents;
        Ok(())
    }
}

// 2 vendors
// Basic gateway: implements Gateway only
struct BasicBank;

impl Gateway for BasicBank {
    fn raw_authorize(&mut self, c: u64) {
        println!("[bank] authorize {c}");
    }
    fn raw_capture(&mut self, c: u64) {
        println!("[bank] capture {c}");
    }
    fn raw_cancel(&mut self) {
        println!("[bank] cancel");
    }
    fn raw_refund(&mut self, c: u64) {
        println!("[bank] refund {c}");
    }
}

// Full-featured gateway: Gateway + PartialGateway
struct StripeLike;

impl Gateway for StripeLike {
    fn raw_authorize(&mut self, c: u64) {
        println!("[stripe] authorize {c}");
    }
    fn raw_capture(&mut self, c: u64) {
        println!("[stripe] capture {c}");
    }
    fn raw_cancel(&mut self) {
        println!("[stripe] cancel");
    }
    fn raw_refund(&mut self, c: u64) {
        println!("[stripe] refund {c}");
    }
}

impl PartialGateway for StripeLike {
    fn raw_capture_partial(&mut self, c: u64) {
        println!("[stripe] capture partial {c}");
    }
    fn raw_refund_partial(&mut self, c: u64) {
        println!("[stripe] refund partial {c}");
    }
}

// Generic functions

// Any vendor, any state where cancelling is allowed (Pending or Authorized)
fn abort<G: Gateway, S: Cancellable>(p: Payment<G, S>) -> Payment<G, Cancelled> {
    p.cancel()
}

// Only partial-capable vendors, only a Captured payment
fn refund_half<G: PartialGateway>(p: &mut Payment<G, Captured>) {
    let half = p.amount_cents / 2;
    p.refund_partial(half).unwrap();
}

fn example() {
    // StripeLike: the full matrix is available
    let p = Payment::new(StripeLike, 10_000).authorize();
    let mut p = p.capture_partial(8_000);
    refund_half(&mut p);
    let _done = p.refund();

    // BasicBank: the basic flow only
    let p = Payment::new(BasicBank, 5_000).authorize().capture();
    let _done = p.refund();

    // Zero cost: the state and the ZST gateway add no bytes
    assert_eq!(size_of::<Payment<BasicBank, Captured>>(), size_of::<u64>());
}
