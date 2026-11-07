use std::any::Any;

fn describe(value: &dyn Any) {
    if let Some(n) = value.downcast_ref::<i32>() {
        println!("an i32: {n}");
    } else if let Some(s) = value.downcast_ref::<String>() {
        println!("a String: {s}");
    } else {
        println!("some other type");
    }
}

fn main() {
    describe(&42i32);
    describe(&String::from("hi"));
    describe(&3.14f64);
}
