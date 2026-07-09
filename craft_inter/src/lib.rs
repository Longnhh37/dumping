use crate::{
    cli::{Cli, CliMode},
    error::RloxErr,
};

mod ast;
mod cli;
mod driver;
mod error;
mod lexer;
mod parser;
mod repl;
mod scanner;

pub fn execute() -> Result<(), RloxErr> {
    let mode = Cli::parse_mode()?;

    match mode {
        CliMode::FromFile(path) => driver::run_file(path)?,
        CliMode::Repl => repl::repl()?,
    }

    Ok(())
}
