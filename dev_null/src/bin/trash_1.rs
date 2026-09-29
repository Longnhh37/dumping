fn main() {
    let a: usize = 3;
    let b: isize = 8;
    match a.checked_add_signed(b) {
        Some(v) => println!("{v}"),
        None => println!("overflow"),
    };
}
