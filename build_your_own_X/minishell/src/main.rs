fn main() {
    if let Err(e) = codecrafters_shell::repl::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

