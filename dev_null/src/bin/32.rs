use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("before");
        if true {
            panic!("something broke");
        }
        42
    });

    match result {
        Ok(value) => println!("closure returned {value}"),
        Err(_payload) => println!("closure panicked, but we are still alive"),
    }
}
