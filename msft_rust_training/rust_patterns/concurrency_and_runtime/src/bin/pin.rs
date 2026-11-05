use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
    data: String,
    ptr: *const String,  // points at the `data` field above
    _pin: PhantomPinned, // opts out of Unpin
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        // Step 1: build the struct with a null ptr (we don't know the final address yet)
        let val = SelfRef {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        };

        // Step 2: put it on the heap and pin it. From here the address is fixed.
        let mut boxed = Box::pin(val);

        // Step 3: now that the address is stable, take a pointer to `data`
        let self_ptr: *const String = &boxed.data;

        // Step 4: write the pointer back into the struct
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).ptr = self_ptr;
        }
        boxed
    }
}

fn main() {}
