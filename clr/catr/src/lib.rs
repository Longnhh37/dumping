use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

// ============================
// CLI layer
// ============================

#[derive(Parser, Debug)]
#[command(name = "catr", version, about = "Rust cat")]
struct Args {
    /// Input files ("-" means stdin)
    #[arg(required = true)]
    files: Vec<String>,

    /// Number all lines
    #[arg(short = 'n', long = "number")]
    number_lines: bool,

    /// Number nonblank lines only (overrides -n)
    #[arg(short = 'b', long = "number-nonblank", overrides_with = "number_lines")]
    number_nonblank_lines: bool,
}

// ============================
// Business Config
// ============================

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    numbering: LineNumbering,
}

#[derive(Debug, Clone, Copy)]
enum LineNumbering {
    None,
    All,
    NonBlank,
}

// ============================
// Parse args -> Config
// ============================

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();

    let numbering = if args.number_nonblank_lines {
        LineNumbering::NonBlank
    } else if args.number_lines {
        LineNumbering::All
    } else {
        LineNumbering::None
    };

    Ok(Config {
        files: args.files,
        numbering,
    })
}

// ============================
// Core logic
// ============================

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        if filename == "-" {
            let stdin = io::stdin();
            let handle = stdin.lock();

            if let Err(e) = process(handle, config.numbering) {
                eprintln!("-: {}", e);
            }
        } else {
            match File::open(&filename) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    if let Err(e) = process(reader, config.numbering) {
                        eprintln!("{}: {}", filename, e);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", filename, e);
                }
            }
        }
    }
    Ok(())
}

// ============================
// Line processing
// ============================

fn process<R: BufRead>(reader: R, numbering: LineNumbering) -> MyResult<()> {
    let mut line_no = 1;

    for line in reader.lines() {
        let line = line?;
        let is_blank = line.trim().is_empty();

        match numbering {
            LineNumbering::None => println!("{}", line),
            LineNumbering::All => {
                println!("{:>6}\t{}", line_no, line);
                line_no += 1;
            }
            LineNumbering::NonBlank => {
                if is_blank {
                    println!();
                } else {
                    println!("{:>6}\t{}", line_no, line);
                    line_no += 1;
                }
            }
        }
    }

    Ok(())
}
