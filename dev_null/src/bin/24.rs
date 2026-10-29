// as_ref()         &Option<T>                      Option<&T>
// as_mut()         &mut Option<T>                  Option<&mut T>
// as_deref()       &Option<T> (T: Deref)           Option<&T::Target>
// as_deref_mut()   &mut Option<T> (T: DerefMut)    Option<&mut T::Target>

fn main() {
    let mut s: Option<String> = Some("hi".to_string());
    let b: Option<Box<i32>> = Some(Box::new(5));
    let v: Option<Vec<u8>> = Some(vec![1, 2, 3]);

    // as_ref: keeps the inner type
    let a: Option<&String> = s.as_ref();
    let _ = a;

    // as_deref: String -> str, Box<i32> -> i32, Vec<u8> -> [u8]
    let a: Option<&str> = s.as_deref();
    let bb: Option<&i32> = b.as_deref();
    let vv: Option<&[u8]> = v.as_deref();
    println!("{a:?} {bb:?} {vv:?}");

    // as_mut: mutate the inner value in place, Option stays Some
    if let Some(x) = s.as_mut() {
        x.push_str(" there"); // x: &mut String
    }

    // as_deref_mut: &mut String -> &mut str
    if let Some(x) = s.as_deref_mut() {
        x.make_ascii_uppercase(); // method of str
    }
    println!("{s:?}"); // Some("HI THERE")
}
