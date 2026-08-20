use anyhow::{Result, anyhow};
use clap::Parser;
use rand::{
    SeedableRng,
    rngs::{StdRng, SysRng},
    seq::IndexedRandom,
};
use regex::RegexBuilder;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true, value_name = "FILE", num_args = 1..)]
    files: Vec<String>,

    #[arg(short = 'm', long)]
    pattern: Option<String>,

    #[arg(short, long)]
    ignore_case: bool,

    #[arg(long)]
    seed: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let re = if let Some(pattern) = args.pattern {
        Some(
            RegexBuilder::new(&pattern)
                .case_insensitive(args.ignore_case)
                .build()
                .map_err(|_| anyhow!(r#"Invalid --pattern "{}""#, pattern))?,
        )
    } else {
        None
    };

    let mut fortunes = Vec::new();

    for file in &args.files {
        for entry in WalkDir::new(file) {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("{}: {}", file, err);
                    continue;
                }
            };

            if entry.file_type().is_file() {
                match read_fortunes(entry.path()) {
                    Ok(f) => fortunes.extend(f),
                    Err(e) => {
                        eprintln!("{}: {}", entry.path().display(), e);
                        continue;
                    }
                }
            }
        }
    }

    if fortunes.is_empty() {
        return Err(anyhow!("No fortunes found"));
    }

    match &re {
        Some(re) => {
            for fortune in fortunes {
                if re.is_match(&fortune) {
                    println!("{}", fortune);
                    println!("%");
                }
            }
        }

        None => {
            let mut rng = match args.seed {
                Some(s) => StdRng::seed_from_u64(s),
                None => StdRng::try_from_rng(&mut SysRng).unwrap(),
            };

            let chosen = fortunes
                .choose(&mut rng)
                .ok_or(anyhow!("No fortunes found"))?;

            println!("{}", chosen);
        }
    }

    Ok(())
}

fn read_fortunes<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;

    Ok(content
        .split("\n%\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect())
}
