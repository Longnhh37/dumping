#[derive(Default, Clone)]
struct RequestConfig {
    method: String,
    url: String,
    validate_status: bool,
}

fn main() {
    let config = RequestConfig {
        method: "get".into(),
        url: "/users".into(),
        ..Default::default()
    };

    // Struct update syntax: copy remaining fields from `config`
    let merged = RequestConfig {
        validate_status: true,
        ..config
    };

    println!("{} {} {}", merged.method, merged.url, merged.validate_status);
}
