//! Notes: Vec vs bumpalo::Bump vs typed_arena::Arena vs slab::Slab
//!
//! | Feature               | Vec<T>      | Bump          | Arena<T>     | Slab<T>               |
//! |-----------------------|-------------|---------------|--------------|----------------- -----|
//! | Types per container   | 1           | many          | 1            | 1                     |
//! | Access by             | index       | returned &mut | returned &mut| key (usize)           |
//! | Stable references     | no (realloc)| yes           | yes          | no (but key is stable)|
//! | Remove single item    | O(n)        | impossible    | impossible   | O(1)                  |
//! | Frees memory          | on drop     | whole arena   | whole arena  | on drop (slots reused)|
//! | Runs Drop on items    | yes         | NO (default)  | yes          | yes                   |
//! | alloc/insert takes    | &mut self   | &self         | &self        | &mut self             |

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bumpalo::Bump;
use slab::Slab;
use typed_arena::Arena;

/// `Bump`: bump-pointer allocator.
///
/// - Allocation = move a pointer forward (a few instructions), very fast.
/// - Can hold MANY different types in the same arena.
/// - `alloc` takes `&self`, so many references can be alive at once.
/// - Individual objects cannot be freed; everything is freed together
///   when the `Bump` is dropped or when `reset()` is called (`&mut self`).
/// - When a chunk is full a new chunk is added; old chunks never move,
///   so existing references stay valid.
/// - Does NOT run `Drop` on items by default (e.g. the heap buffer of a
///   `String` would leak). Only put `Copy` / POD types / `&str` in it, or use
///   `bumpalo::collections::{Vec, String}` / `bumpalo::boxed::Box`
///   (cargo features `collections` / `boxed`).
/// - Use cases: AST of one parse, per-request data, per-frame allocator.
fn bump_arena_for_different_types() {
    let bump = Bump::new();

    let a: &mut u32 = bump.alloc(1);
    let s: &str = bump.alloc_str("hello_world");
    let pair: &mut (i32, f64) = bump.alloc((7, 2.9));

    *a += 1;
    pair.0 += 12;

    assert_eq!(*a, 2);
    assert_eq!(s, "hello_world");
    assert_eq!(pair.0, 19);
    // everything is freed at once here (no per-item Drop)
}

/// `typed_arena::Arena<T>`: arena for exactly ONE type.
///
/// - `alloc(&self, T) -> &mut T`, references live as long as the arena.
/// - Only one type `T`, but in exchange it DOES run `Drop` on every item
///   when the arena is dropped.
/// - Individual items cannot be freed.
/// - Great for graph / tree / cyclic structures: no need for
///   `Rc<RefCell<..>>`, nodes just hold `&'a Node<'a>`.
///   Interior mutability (`RefCell`) is needed to link nodes after creation.
fn typed_arena_for_one_type() {
    struct Node<'a> {
        val: i32,
        edges: RefCell<Vec<&'a Node<'a>>>,
    }

    let arena = Arena::new();

    // Annotate as `&Node` to get a shared (Copy) reference.
    let a: &Node = arena.alloc(Node {
        val: 1,
        edges: RefCell::new(vec![]),
    });
    let b: &Node = arena.alloc(Node {
        val: 2,
        edges: RefCell::new(vec![]),
    });

    // A cycle: a -> b -> a
    a.edges.borrow_mut().push(b);
    b.edges.borrow_mut().push(a);

    assert_eq!(a.edges.borrow()[0].val, 2);
    assert_eq!(b.edges.borrow()[0].val, 1);
    // all nodes are dropped here (Drop IS run)
}

/// `slab::Slab<T>`: a `Vec` with a free list, returns keys instead of references.
///
/// - Internally `Vec<Entry<T>>`, where an entry is `Occupied(T)` or
///   `Vacant(next_free_index)`.
/// - `insert` returns a `usize` key; `remove(key)` and `slab[key]` are O(1).
/// - Removing leaves a hole that goes into the free list and is reused by the
///   next `insert`.
/// - Keys are STABLE (removing other items does not change them), but
///   references are NOT stable: `insert` may realloc the inner Vec.
/// - Keys are REUSED after removal (ABA problem): an old key may silently
///   point to a new item. Use `slotmap` if you need generation counters.
/// - Use cases: connection table (`token -> connection`), entity storage,
///   any place that needs small dense IDs and fast removal.
fn slab_for_free_list() {
    let mut slab = Slab::new();

    let k1 = slab.insert("alice");
    let k2 = slab.insert("bob");

    // O(1) removal, leaves a hole in the free list
    assert_eq!(slab.remove(k1), "alice");
    assert!(!slab.contains(k1));

    // the freed slot is reused, so the key is reused too
    let k3 = slab.insert("carol");
    assert_eq!(k3, k1);

    // k2 is still valid
    assert_eq!(slab[k2], "bob");

    // ABA: the old key k1 now points to "carol", not "alice"
    assert_eq!(slab.get(k1), Some(&"carol"));

    // iteration skips vacant slots
    for (key, name) in &slab {
        println!("{key}: {name}");
    }
}

/// Shows the `Drop` difference: `Bump` does not drop items, `Arena` does.
fn drop_behavior() {
    struct DropCounter(Rc<Cell<u32>>);

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let count = Rc::new(Cell::new(0));

    {
        let bump = Bump::new();
        let _ = bump.alloc(DropCounter(Rc::clone(&count)));
    }
    assert_eq!(count.get(), 0); // Bump never ran Drop

    {
        let arena = Arena::new();
        let _ = arena.alloc(DropCounter(Rc::clone(&count)));
    }
    assert_eq!(count.get(), 1); // Arena ran Drop
}

fn example() {
    bump_arena_for_different_types();
    typed_arena_for_one_type();
    slab_for_free_list();
    drop_behavior();
}
