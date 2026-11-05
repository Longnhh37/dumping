// -----------------------------------------------------
// Drop order
// -----------------------------------------------------
struct Label(&'static str);

impl Drop for Label {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

struct Server {
    listener: Label, // Dropped 1st
    handler: Label,  // Dropped 2nd
    logger: Label,   // Dropped 3rd
}
// Fields drop top-to-bottom (declaration order).
// This matters when fields reference each other or hold resources.

fn main() {
    let a = Label("first"); // Declared first
    let b = Label("second"); // Declared second
    let c = Label("third"); // Declared third
}
// Output:
//   Dropping third    ← locals drop in REVERSE declaration order
//   Dropping second
//   Dropping first

// -----------------------------------------------------
// Manual drop
// -----------------------------------------------------
use std::mem::ManuallyDrop;

// Use case 1: Prevent double-free in unsafe code
struct TwoPhaseBuffer {
    data: ManuallyDrop<Vec<u8>>,
    committed: bool,
}

impl TwoPhaseBuffer {
    fn new(capacity: usize) -> Self {
        TwoPhaseBuffer {
            data: ManuallyDrop::new(Vec::with_capacity(capacity)),
            committed: false,
        }
    }

    fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    fn commit(&mut self) {
        self.committed = true;
        println!("Committed {} bytes", self.data.len());
    }
}

impl Drop for TwoPhaseBuffer {
    fn drop(&mut self) {
        if !self.committed {
            println!("Rolling back — dropping uncommitted data");
        }
        unsafe {
            ManuallyDrop::drop(&mut self.data);
        }
    }
}

// Use case 2: Intentional leak (e.g., global singletons)
fn leaked_string_boxed() -> &'static str {
    let s = String::from("lives forever");
    Box::leak(s.into_boxed_str())
}
