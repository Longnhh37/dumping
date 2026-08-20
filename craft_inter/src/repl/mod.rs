use std::io::{self, Write};

use crate::{driver, error::RloxErr};

pub fn repl() -> Result<(), RloxErr> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;

        if n == 0 {
            break;
        }

        let input = line.trim();

        if matches!(input, "quit" | "q" | "exit") {
            break;
        }

        driver::run_source(input)?;
    }

    Ok(())
}
