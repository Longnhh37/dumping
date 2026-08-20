
use anyhow::Result;
use clap::{ArgAction, Parser, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "findr", version, about = "find rust")]
struct Args {
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    #[arg(
        short = 't', 
        long = "type",
        value_name = "TYPE",
        value_parser(clap::value_parser!(EntryType)),
        action(ArgAction::Append),
        num_args(0..),
    )]
    entry_types: Vec<EntryType>,


    #[arg(
        short = 'n', 
        long = "name",
        value_name = "NAME",
        value_parser(Regex::new),
        action(ArgAction::Append),
        num_args(0..)


    )]
    names: Vec<Regex>,
}

#[derive(Clone, Debug, ValueEnum)]
enum EntryType {
    #[value(name = "f")]
    File,
    #[value(name = "d")]
    Dir,
    #[value(name = "l")]
    Link,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::File => entry.file_type().is_file(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::Link => entry.file_type().is_symlink(),
        })
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args.names
                    .iter()
                    .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
