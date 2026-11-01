// ✅ Dyn compatible — can be used as dyn Drawable
trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

let shapes: Vec<Box<dyn Drawable>> = vec![/* ... */]; // ✅ Works

// ❌ NOT dyn compatible — mentions Self outside the receiver
trait Cloneable {
    fn clone_self(&self) -> Self;
    //                      ^^^^ "...because method `clone_self` references
    //                            the `Self` type in its return type"
}
// let items: Vec<Box<dyn Cloneable>> = ...; // ❌ Compile error

// ❌ Same rule, argument position — this is why PartialEq isn't dyn compatible
trait Comparable {
    fn equals(&self, other: &Self) -> bool;
    //                       ^^^^ "...references the `Self` type in this parameter"
}

// ⚠️ Wrapping in Box does NOT help — Box<Self> still names Self
trait Spawner {
    fn spawn(&self) -> Box<Self>; // ❌ Still not dyn compatible
}

// ✅ Return Box<dyn Trait> instead — that's a concrete type, not Self.
// This is the standard "clone through a trait object" idiom:
trait CloneableDyn {
    fn clone_box(&self) -> Box<dyn CloneableDyn>;
}

// ❌ NOT dyn compatible — generic method
trait Converter {
    fn convert<T>(&self) -> T;
    //        ^^^ The vtable can't contain infinite monomorphizations
}

// ✅ Dyn compatible — a generic LIFETIME is fine, only type params are barred
trait Tokenizer {
    fn first_token<'a>(&self, input: &'a str) -> &'a str;
    //            ^^^^ One vtable slot is enough: lifetimes are erased
}

// ❌ NOT dyn compatible — associated function with no receiver
trait Factory {
    fn create() -> Self;
    // "...because associated function `create` has no `self` parameter"
}

// ✅ Same function, opted out of the vtable — the trait is dyn compatible again
trait FactoryFixed {
    fn describe(&self) -> String;      // dispatchable
    fn create() -> Self where Self: Sized; // excluded from the vtable
}

// ❌ NOT dyn compatible — inherited from a supertrait that isn't.
//    Nothing is wrong with Derived itself; the compiler points at Base::make.
trait Base {
    fn make() -> Self;
}
trait Derived: Base {
    fn show(&self);
}
// let d: &dyn Derived = ...; // ❌ Compile error, blamed on `Base::make`

// ❌ NOT dyn compatible — an associated const has no vtable representation
trait Sensor {
    const MAX: f64;
    //    ^^^ "...because it contains associated const `MAX`"
    fn read(&self) -> f64;
}
// Workaround: make it a method — `fn max_reading(&self) -> f64`. Note there is
// NO `where Self: Sized` escape hatch here; `const MAX: f64 where Self: Sized;`
// isn't even accepted syntax on stable (generic const items are unstable).

// ❌ NOT dyn compatible — a GAT is a family of types, not one type
trait LendingIterator {
    type Item<'a> where Self: 'a;
    //   ^^^^ "...because it contains generic associated type `Item`"
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
// This is the same LendingIterator from the GATs section above: the price of
// a lending iterator is that `dyn LendingIterator` can never exist.

// ❌ NOT dyn compatible — opaque return type (RPITIT)
trait Container {
    fn items(&self) -> impl Iterator<Item = u32>;
    //                 ^^^^ "...references an `impl Trait` type in its return type"
}

// ❌ NOT dyn compatible — `async fn` is the same problem with sugar on top
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
    // "...because method `get` is `async`"
}
// Workaround for both: erase the type yourself and return a concrete boxed
// trait object — `Box<dyn Iterator<Item = u32> + '_>` and
// `Pin<Box<dyn Future<Output = Option<String>> + '_>>` are ordinary types,
// so they get ordinary vtable slots.

// ⚠️ `self` by value is NOT a dispatchable receiver — it implies
//    `where Self: Sized`. The trait stays dyn compatible, but the method
//    simply isn't in the vtable and can't be called through `dyn Trait`:
trait Consume {
    fn describe(&self) -> String; // in the vtable
    fn consume(self) -> String;   // implicitly `where Self: Sized`
}
// let t: &dyn Consume = &token;  // ✅ the trait object is fine
// boxed.consume();               // ❌ error[E0161]: cannot move a value
//                                //    of type `dyn Consume`
