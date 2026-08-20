use rlox::execute;

fn main() {
    if let Err(e) = execute() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
