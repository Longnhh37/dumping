use std::marker::PhantomPinned;
use std::pin::Pin;

// A self-referential struct
struct SelfRef {
    data: String,
    ptr: *const String, // point to 'data' above
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        let val = SelfRef {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(val);

        let self_ptr: *const String = &boxed.data;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).ptr = self_ptr;
        }
        boxed
    }

    fn data(&self) -> &str {
        &self.data
    }

    fn ptr_data(&self) -> &str {
        unsafe { &*self.ptr }
    }
}

fn main() {
    let pinned = SelfRef::new("Hellow world");
    assert_eq!(pinned.data(), pinned.ptr_data());
}
