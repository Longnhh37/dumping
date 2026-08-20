use std::{fs::File, io::Read, path::PathBuf};

use crate::{error::RloxErr, parser::Parser, scanner::Scanner};

pub fn run_file(path: PathBuf) -> Result<(), RloxErr> {
    let mut file = File::open(path)?;

    let mut source = String::new();
    file.read_to_string(&mut source)?;

    run_source(&source)?;
    Ok(())
}

pub fn run_source(source: &str) -> Result<(), RloxErr> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let expr = Parser::new(tokens).parse()?;

    println!("{expr:#?}");

    Ok(())
}
