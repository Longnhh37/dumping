use std::collections::HashMap;

// Flexible: accepts a borrowed str from anywhere
fn greet(name: Option<&str>) {
    println!("hello, {}", name.unwrap_or("stranger"));
}

// Rigid: caller must own exactly Option<String>
fn greet_rigid(name: &Option<String>) {
    println!("hello, {}", name.as_deref().unwrap_or("stranger"));
}

fn main() {
    let owned: Option<String> = Some("Alice".to_string());
    let map = HashMap::from([(1, "Bob".to_string())]);
    let names = vec!["Charlie".to_string()];

    greet(owned.as_deref()); // from Option<String>
    greet(map.get(&1).map(String::as_str)); // from HashMap::get
    greet(names.first().map(String::as_str)); // from slice/Vec
    greet(Some("literal")); // from a &'static str, no allocation

    greet_rigid(&owned);
    // greet_rigid(&Some("literal")); // error: expected Option<String>, found Option<&str>
}
