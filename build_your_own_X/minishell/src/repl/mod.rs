use std::sync::{Arc, RwLock};

use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, Editor};

use crate::completion::ShellHelper;
use crate::exec;
use crate::expander;
use crate::parser::{self, ParseError};
use crate::runtime::{ExecStatus, ShellState};

pub fn run() -> Result<(), ReadlineError> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let state = Arc::new(RwLock::new(ShellState::new()));

    let mut rl = Editor::<ShellHelper, DefaultHistory>::with_config(config)?;
    rl.set_helper(Some(ShellHelper::new(Arc::clone(&state))));

    let history_path = std::env::var("HOME")
        .ok()
        .map(|h| format!("{h}/.myshell_history"));

    if let Some(ref p) = history_path {
        let _ = rl.load_history(p);
    }

    loop {
        let input = match rl.readline("$ ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        };

        if !input.trim().is_empty() {
            let _ = rl.add_history_entry(input.as_str());
        }

        let ast = match parser::parse(&input) {
            Ok(a) => a,
            Err(ParseError::EmptyInput) | Err(ParseError::WhitespaceOnly) => {
                state.write().unwrap().last_status = 0;
                continue;
            }
            Err(e) => {
                eprintln!("{}", e);
                state.write().unwrap().last_status = 2;
                continue;
            }
        };
        let expanded = {
            let s = state.read().unwrap();
            expander::expand(ast, &s)
        };

        let expanded = match expanded {
            Ok(cmds) => cmds,
            Err(e) => {
                eprintln!("{}", e);
                state.write().unwrap().last_status = 1;
                continue;
            }
        };

        let status = exec::run(expanded, &mut state.write().unwrap());

        match status {
            ExecStatus::Code(c) => state.write().unwrap().last_status = c,
            ExecStatus::Exit(c) => {
                if let Some(ref p) = history_path {
                    let _ = rl.save_history(p);
                }
                std::process::exit(c);
            }
        }
    }

    if let Some(ref p) = history_path {
        let _ = rl.save_history(p);
    }

    Ok(())
}
