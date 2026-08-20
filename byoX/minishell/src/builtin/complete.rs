use clap::{Arg, ArgAction, Command};

use crate::runtime::{CompSpec, ExecStatus, ShellState};

pub fn run(args: &[String], state: &mut ShellState) -> ExecStatus {
    let cmd = Command::new("complete")
        .no_binary_name(true)
        .disable_help_flag(true)
        .arg(Arg::new("wordlist").short('W').value_name("WORDLIST"))
        .arg(Arg::new("command").short('C').value_name("CMD"))
        .arg(Arg::new("file").short('f').action(ArgAction::SetTrue))
        .arg(Arg::new("dir").short('d').action(ArgAction::SetTrue))
        .arg(Arg::new("print").short('p').action(ArgAction::SetTrue))
        .arg(Arg::new("names").num_args(0..).trailing_var_arg(true));

    let matches = match cmd.try_get_matches_from(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return ExecStatus::Code(1);
        }
    };

    // -p: print registered spec for the given command name(s)
    if matches.get_flag("print") {
        let names: Vec<&String> = matches
            .get_many::<String>("names")
            .unwrap_or_default()
            .collect();

        for name in &names {
            match state.comp_registry.get(*name) {
                Some(spec) => println!("{}", format_spec(spec, name)),
                None => eprintln!("complete: {}: no completion specification", name),
            }
        }

        return ExecStatus::Code(0);
    }

    let spec = if let Some(wordlist) = matches.get_one::<String>("wordlist") {
        let words = wordlist.split_whitespace().map(str::to_owned).collect();
        CompSpec::List(words)
    } else if let Some(cmd_path) = matches.get_one::<String>("command") {
        CompSpec::Command(cmd_path.clone())
    } else if matches.get_flag("file") {
        CompSpec::File
    } else if matches.get_flag("dir") {
        CompSpec::Dir
    } else {
        CompSpec::File
    };

    let names: Vec<&String> = matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .collect();

    for name in names {
        state.comp_registry.insert(name.clone(), spec.clone());
    }

    ExecStatus::Code(0)
}

fn format_spec(spec: &CompSpec, name: &str) -> String {
    match spec {
        CompSpec::List(words) => format!("complete -W '{}' {}", words.join(" "), name),
        CompSpec::Command(cmd) => format!("complete -C '{}' {}", cmd, name),
        CompSpec::File => format!("complete -f {}", name),
        CompSpec::Dir => format!("complete -d {}", name),
    }
}
