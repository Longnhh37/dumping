use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "echor", version, about = "Rust minimal echo")]
struct Args {
    text: Vec<String>,

    #[arg(short = 'n')]
    omit_newline: bool,
}

pub fn run() -> io::Result<()> {
    let args = Args::parse();

    let output = args.text.join(" ");
    let mut stdout = io::stdout();

    if args.omit_newline {
        write!(stdout, "{}", output)?;
    } else {
        writeln!(stdout, "{}", output)?;
    }

    Ok(())
}

