use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::path::Path;

struct MyBox<T>(T);

// Deref
// a smart pointer to 1 Target
// 1 type can deref into 1 type only
// deref coercion: &String -> &str, &Box<T> -> &T, &Vec<T> -> &[T]
// anti-pattern: use Deref as inheritance
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn greet(name: &str) {
    println!("Hello, {name}");
}

// AsRef
// generic trait. 1 type can impl for multilpe T
// No implicit conversion, call with .as_ref()
// Ex: String -> AsRef<str>, AsRef<[u8]>, AsRef<Path>, AsRef<OsStr>,

// Accepts &str, String, PathBuf, &Path, OsString, ...
fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

// Borrow<T>
// borrowed form keeps Eq/Hash/Ord
// stricter contract: x: Owned, y = x.borrow() -> Eq, Ord, Hash of x == y

fn main() {
    // Deref
    let b = MyBox(String::from("Rust"));
    // &MyBox<String> -> &String -> &str (chained deref coercion)
    greet(&b);

    // AsRef
    file_exists("Cargo.toml");
    file_exists(String::from("Cargo.toml"));
    file_exists(Path::new("Cargo.toml"));

    // Borrow
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("answer".to_owned(), 42);
    // Lookup with &str, no String allocation needed
    assert_eq!(map.get("answer"), Some(&42));
}
