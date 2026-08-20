use crate::runtime::{ExecStatus, ShellState, is_valid_identifier};

pub fn run(args: &[String], state: &mut ShellState) -> ExecStatus {
    if args.is_empty() {
        let mut exported: Vec<_> = state.exported.iter().collect();
        exported.sort();

        for k in exported {
            let v = state.vars.get(k).map(|s| s.as_str()).unwrap_or("");
            println!(r#"declare -x {}="{}""#, k, v);
        }
        return ExecStatus::Code(0);
    }

    for arg in args {
        match arg.split_once('=') {
            Some((name, value)) => {
                if !is_valid_identifier(name) {
                    eprintln!("export: '{}': not a valid identifier", name);
                    return ExecStatus::Code(1);
                }
                state.export_var(name.to_string(), value.to_string());
            }

            None => {
                if !is_valid_identifier(arg) {
                    eprintln!("export: '{}': not a valid identifier", arg);
                    return ExecStatus::Code(1);
                }

                let value = state.get_var(arg).unwrap_or_default();
                state.export_var(arg.clone(), value);
            }
        }
    }

    ExecStatus::Code(0)
}
