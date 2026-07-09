fn main() {
    match wcr::run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

