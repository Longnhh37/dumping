use anyhow::{Result, anyhow};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    path1: String,

    #[arg(required = true)]
    path2: String,

    #[arg(short('1'))]
    omit_col1: bool,

    #[arg(short('2'))]
    omit_col2: bool,

    #[arg(short('3'))]
    omit_col3: bool,

    #[arg(short, long)]
    insensitive: bool,

    #[arg(short, long, default_value = "\t")]
    delimiter: String,
}

// ---------------------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    println!(car)
}

// ---------------------------------------------------------------
fn run(args: Args) -> Result<()> {
    if args.path1 == args.path2 && args.path1 == "-" {
        return Err(anyhow!(r#"Both input files cannot be STDIN ("-")"#));
    }

    let file1 = sort_file(&args.path1, args.insensitive)
        .map_err(|e| anyhow!(r#"{}: {}"#, &args.path1, e))?;

    let file2 = sort_file(&args.path2, args.insensitive)
        .map_err(|e| anyhow!(r#"{}: {}"#, &args.path2, e))?;


    let mut i = 0;
    let mut j = 0;

    let d1 = if args.omit_col1 { "" } else { &args.delimiter };
    let d2 = if args.omit_col2 { "" } else { &args.delimiter };

    while i < file1.len() && j < file2.len() {
        match file1[i].cmp(&file2[j]) {
            std::cmp::Ordering::Less => {
                if !args.omit_col1 {
                    println!("{}", file1[i]);
                }
                i += 1;
            }
            std::cmp::Ordering::Equal => {
                if !args.omit_col3 {
                    println!("{}{}{}", d1, d2, file1[i]); 
                }
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => {
                if !args.omit_col2 {
                    println!("{}{}", d1, file2[j]); 
                }
                j += 1;
            }
        }
    }

    if !args.omit_col1 && i < file1.len() {
        for line in &file1[i..] {
            println!("{}", line);
        }
    }

    if !args.omit_col2 && j < file2.len() {
        for line in &file2[j..] {
            println!("{}{}", d1, line);
        }
    }

    Ok(())
}

// ---------------------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// ---------------------------------------------------------------
fn sort_file(path: &str, ignore_case: bool) -> Result<Vec<String>> {
    let reader = open(path)?;
    let mut file = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if ignore_case {
            file.push(line.to_lowercase());
        } else {
            file.push(line);
        }
    }

    file.sort_unstable();
    Ok(file)
}
