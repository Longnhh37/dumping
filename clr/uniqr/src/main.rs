fn main() {
    match uniqr::run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
