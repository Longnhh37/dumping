use std::cell::RefCell;
use std::marker::PhantomData;

/// A handle branded to a specific arena instance.
struct ArenaHandle<'arena> {
    index: usize,
    _phantom: PhantomData<&'arena ()>,
}

/// An arena that brands each handle with its unique lifetime.
/// Invariant over 'arena — prevents shortening the brand to match another arena.
struct Arena<'arena> {
    data: RefCell<Vec<String>>,
    _brand: PhantomData<*mut &'arena ()>,
}

/// Create an arena and pass it to a closure.
/// Each call gets a unique, opaque lifetime that can't be forged.
fn with_arena<R>(f: impl for<'arena> FnOnce(&Arena<'arena>) -> R) -> R {
    let arena = Arena {
        data: RefCell::new(Vec::new()),
        _brand: PhantomData,
    };
    f(&arena)
}

impl<'arena> Arena<'arena> {
    /// Allocate a string and return a branded handle
    fn alloc(&self, value: String) -> ArenaHandle<'arena> {
        let mut data = self.data.borrow_mut();
        let index = data.len();
        data.push(value);
        ArenaHandle { index, _phantom: PhantomData }
    }

    /// Look up by handle — only accepts handles from THIS arena
    fn get(&self, handle: &ArenaHandle<'arena>) -> String {
        let data = self.data.borrow();
        data[handle.index].clone()
    }
}

fn main() {
    with_arena(|arena1| {
        with_arena(|arena2| {
            let handle1 = arena1.alloc("hello".to_string());
            println!("{}", arena1.get(&handle1)); // ✅

            // Can't use handle1 with a different arena — compile-time error
            arena2.get(&handle1); // ❌ borrowed data escapes outside of closure
        });
    });
}
