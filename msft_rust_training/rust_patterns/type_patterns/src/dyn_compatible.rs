// Dyn compatibility walkthrough (edition 2024, Rust 1.90+)
// Every "NOT dyn compatible" example is commented out so the file compiles.
// Uncomment one at a time to see the compiler error.

use std::collections::HashMap;
use std::future::Future;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

// ============================================================================
// 0. Background: how `dyn Trait` works
// ============================================================================
// `Box<dyn Drawable>` is a fat pointer: [data ptr | vtable ptr].
// The concrete type is erased. Method calls go through the vtable.
// A method can live in the vtable only if:
//   1. its signature is fixed (does not depend on the erased type),
//   2. the number of slots is finite and known,
//   3. it can be called through a pointer (it has a receiver).

// ============================================================================
// 1. Normal dyn compatible trait
// ============================================================================
// Fixed signatures, has receiver, no generics -> 2 vtable slots.
trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

struct Circle(f64);
struct Square(f64, f64);

impl Drawable for Circle {
    fn draw(&self) {
        println!("circle r={}", self.0);
    }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (-self.0, -self.0, self.0, self.0)
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("square {}x{}", self.0, self.1);
    }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (0.0, 0.0, self.0, self.1)
    }
}

// ============================================================================
// 2. Violation of rule 1: `Self` outside the receiver
// ============================================================================
// After type erasure, `Self` is unknown: size, identity, everything.

// NOT dyn compatible: return type `Self` has unknown size.
// trait Cloneable {
//     fn clone_self(&self) -> Self;
// }

// NOT dyn compatible: `other: &Self` cannot be checked to be the same
// concrete type as `self` once both are erased. This is why `PartialEq`
// is not dyn compatible.
// trait Comparable {
//     fn equals(&self, other: &Self) -> bool;
// }

// NOT dyn compatible: `Box<Self>` still names `Self`.
// trait Spawner {
//     fn spawn(&self) -> Box<Self>;
// }

// FIX: return `Box<dyn Trait>`. It is one concrete type for every implementor.
// This is the standard "clone through a trait object" idiom.
trait Shape {
    fn name(&self) -> String;
    fn clone_box(&self) -> Box<dyn Shape>;
}

#[derive(Clone)]
struct Dot;

impl Shape for Dot {
    fn name(&self) -> String {
        "dot".into()
    }
    fn clone_box(&self) -> Box<dyn Shape> {
        // Here we know Self = Dot, so a normal clone works.
        Box::new(self.clone())
    }
}

// Bonus: with clone_box we can make `Box<dyn Shape>` itself implement Clone.
impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// ============================================================================
// 3. Violation of rule 2: generic type parameter on a method
// ============================================================================
// Every `T` produces a different monomorphized function, so the vtable would
// need potentially infinite slots.

// NOT dyn compatible:
// trait Converter {
//     fn convert<T>(&self) -> T;
// }

// A generic LIFETIME is fine: lifetimes are erased, so there is only one
// compiled function and one vtable slot.
trait Tokenizer {
    fn first_token<'a>(&self, input: &'a str) -> &'a str;
}

struct Whitespace;

impl Tokenizer for Whitespace {
    fn first_token<'a>(&self, input: &'a str) -> &'a str {
        input.split_whitespace().next().unwrap_or("")
    }
}

// ============================================================================
// 4. Violation of rule 3: associated function without receiver
// ============================================================================
// No `self` means no pointer to follow to find the vtable.

// NOT dyn compatible:
// trait Factory {
//     fn create() -> Self;
// }

// FIX: `where Self: Sized` excludes the method from the vtable.
// `dyn Trait` is `!Sized`, so the method is simply uncallable through `dyn`,
// but still callable on concrete types (`Widget::create()`).
trait FactoryFixed {
    fn describe(&self) -> String; // dispatchable (in the vtable)
    fn create() -> Self
    where
        Self: Sized; // excluded from the vtable
}

struct Widget;

impl FactoryFixed for Widget {
    fn describe(&self) -> String {
        "widget".into()
    }
    fn create() -> Self {
        Widget
    }
}

// ============================================================================
// 5. Inherited from a supertrait
// ============================================================================
// `dyn Derived` carries everything from `Base`, so a bad method in `Base`
// breaks `dyn Derived` even though `Derived` itself is fine.

// NOT dyn compatible (the compiler blames `Base::make`):
// trait Base {
//     fn make() -> Self;
// }
// trait Derived: Base {
//     fn show(&self);
// }

// FIX: repair it at the source, in `Base`.
trait Base {
    fn make() -> Self
    where
        Self: Sized;
}

trait Derived: Base {
    fn show(&self);
}

struct Thing;

impl Base for Thing {
    fn make() -> Self {
        Thing
    }
}

impl Derived for Thing {
    fn show(&self) {
        println!("thing");
    }
}

// ============================================================================
// 6. Associated const
// ============================================================================
// A vtable holds function pointers (plus drop/size/align), not constants.
// There is NO `where Self: Sized` escape hatch for consts on stable.

// NOT dyn compatible:
// trait Sensor {
//     const MAX: f64;
//     fn read(&self) -> f64;
// }

// FIX: make it a method, which becomes a vtable slot.
trait Sensor {
    fn max_reading(&self) -> f64;
    fn read(&self) -> f64;
}

struct Thermometer;

impl Sensor for Thermometer {
    fn max_reading(&self) -> f64 {
        100.0
    }
    fn read(&self) -> f64 {
        36.6
    }
}

// ============================================================================
// 7. GAT (generic associated type)
// ============================================================================
// `Item<'a>` is a family of types (one per lifetime), not a single type, so
// you could never write `dyn LendingIterator<Item = ...>`. No fix exists:
// `dyn LendingIterator` can never exist. Use another design.

// NOT dyn compatible:
// trait LendingIterator {
//     type Item<'a>
//     where
//         Self: 'a;
//     fn next(&mut self) -> Option<Self::Item<'_>>;
// }

// ============================================================================
// 8. `impl Trait` in return position (RPITIT) and `async fn`
// ============================================================================
// `-> impl Trait` means "each implementor picks its own hidden type", so the
// return type (and its size) differs per impl. `async fn` is sugar for
// `-> impl Future<Output = ...>`, same problem.

// NOT dyn compatible:
// trait Container {
//     fn items(&self) -> impl Iterator<Item = u32>;
// }
// trait DataStore {
//     async fn get(&self, key: &str) -> Option<String>;
// }

// FIX: erase the type yourself with a boxed trait object.
// `Box<dyn ...>` is an ordinary sized type, so it gets an ordinary slot.
// Cost: one heap allocation per call (the `async-trait` crate automates this).
trait Container {
    fn items(&self) -> Box<dyn Iterator<Item = u32> + '_>;
}

trait DataStore {
    fn get<'a>(&'a self, key: &'a str) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>>;
}

struct Numbers(Vec<u32>);

impl Container for Numbers {
    fn items(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.0.iter().copied())
    }
}

struct MemStore(HashMap<String, String>);

impl DataStore for MemStore {
    fn get<'a>(&'a self, key: &'a str) -> Pin<Box<dyn Future<Output = Option<String>> + 'a>> {
        Box::pin(async move { self.0.get(key).cloned() })
    }
}

// Minimal executor so we can run a future without tokio.
// `Waker::noop()` is stable since Rust 1.85.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

// ============================================================================
// 9. `self` by value: trait stays dyn compatible, the method is not callable
// ============================================================================
// Moving `self` needs the size, and `dyn Consume` is `!Sized`. So `consume`
// is implicitly `where Self: Sized`: not in the vtable, and uncallable via `dyn`.
trait Consume {
    fn describe(&self) -> String; // in the vtable
    fn consume(self) -> String; // implicitly `where Self: Sized`
}

struct Token(String);

impl Consume for Token {
    fn describe(&self) -> String {
        format!("token({})", self.0)
    }
    fn consume(self) -> String {
        self.0 // moves the String out
    }
}

// ============================================================================
// main: exercise everything that compiles
// ============================================================================
fn main() {
    // 1. Different concrete types in one Vec, dispatched at runtime.
    let shapes: Vec<Box<dyn Drawable>> = vec![Box::new(Circle(1.0)), Box::new(Square(2.0, 3.0))];
    for s in &shapes {
        s.draw();
        println!("  bbox = {:?}", s.bounding_box());
    }

    // 2. Clone through a trait object.
    let original: Box<dyn Shape> = Box::new(Dot);
    let copy = original.clone(); // uses `impl Clone for Box<dyn Shape>`
    println!("{} / {}", original.name(), copy.name());

    // 3. Generic lifetime is fine.
    let tok: &dyn Tokenizer = &Whitespace;
    println!("first token = {:?}", tok.first_token("hello world"));

    // 4. `create` works on the concrete type, `describe` works through dyn.
    let w = Widget::create();
    let dynamic: &dyn FactoryFixed = &w;
    println!("{}", dynamic.describe());
    // let bad = <dyn FactoryFixed>::create(); // error: requires `Self: Sized`

    // 5. `Thing::make()` on concrete, `show()` through dyn.
    let t = Thing::make();
    let d: &dyn Derived = &t;
    d.show();

    // 6. Method instead of const.
    let sensor: Box<dyn Sensor> = Box::new(Thermometer);
    println!("read={} max={}", sensor.read(), sensor.max_reading());

    // 8. Boxed iterator and boxed future.
    let c: Box<dyn Container> = Box::new(Numbers(vec![1, 2, 3]));
    println!("items = {:?}", c.items().collect::<Vec<_>>());

    let store: Box<dyn DataStore> = Box::new(MemStore(HashMap::from([(
        "k".to_string(),
        "v".to_string(),
    )])));
    println!("get(k) = {:?}", block_on(store.get("k")));
    println!("get(x) = {:?}", block_on(store.get("x")));

    // 9. The trait object is fine, `describe` works through dyn...
    let boxed: Box<dyn Consume> = Box::new(Token("abc".into()));
    println!("{}", boxed.describe());
    // ...but `consume` cannot be called through dyn:
    // boxed.consume(); // error[E0161]: cannot move a value of type `dyn Consume`

    // On the concrete type it works fine.
    let concrete = Token("xyz".into());
    println!("{}", concrete.consume());
}
