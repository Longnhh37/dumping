fn unlocked_with_unsafe() {
    unsafe {
        // 1. Dereference a raw pointer
        let ptr: *const i32 = &42;
        let value = *ptr; // Could be a dangling/null pointer

        // 2. Call an unsafe function
        let layout = std::alloc::Layout::new::<u64>();
        let mem = std::alloc::alloc(layout);

        // 3. Access a mutable static variable
        static mut COUNTER: u32 = 0;
        COUNTER += 1; // Data race if multiple threads access

        // 4. Implement an unsafe trait
        // unsafe impl Send for MyType {}

        // 5. Access fields of a union
        // union IntOrFloat { i: i32, f: f32 }
        // let u = IntOrFloat { i: 42 };
        // let f = u.f; // Reinterpret bits — could be garbage
    }
}
