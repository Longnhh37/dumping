use std::{fs, path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use logscope_core::{LogLevel, analyze, build_info, parse_line};

#[derive(Debug, Parser)]
#[command(name = "logscope", version, about = "A small log analysis CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Analyze a timestamp/level/message log file
    Analyze {
        path: PathBuf,
        #[arg(long)]
        level: Option<LogLevelArg>,
    },
    /// Show embedded build metadata
    Info,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum LogLevelArg {
    Info,
    Warn,
    Error,
    Debug,
}

impl From<LogLevelArg> for LogLevel {
    fn from(value: LogLevelArg) -> Self {
        match value {
            LogLevelArg::Info => LogLevel::Info,
            LogLevelArg::Warn => LogLevel::Warn,
            LogLevelArg::Error => LogLevel::Error,
            LogLevelArg::Debug => LogLevel::Debug,
        }
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Info => {
            let info = build_info();
            println!("LogScope {}", env!("CARGO_PKG_VERSION"));
            println!("Git SHA {}", info.git_sha);
            println!("Build timestamp (Unix): {}", info.build_timestamp);
        }
        Command::Analyze { path, level } => {
            let content = fs::read_to_string(path)?;
            let mut entries = Vec::new();
            let mut invalid = 0usize;

            for line in content.lines() {
                match parse_line(line) {
                    Ok(entry) => entries.push(entry),
                    Err(_) => invalid += 1,
                }
            }

            if let Some(level) = level {
                let wanted: LogLevel = level.into();
                entries.retain(|entry| entry.level == wanted);
            }

            let summary = analyze(&entries);

            #[cfg(feature = "json")]
            println!("{}", serde_json::to_string_pretty(&summary)?);

            #[cfg(not(feature = "json"))]
            {
                println!("LogScope Analysis");
                println!("Total entries: {}", summary.total);
                println!("INFO:  {}", summary.info);
                println!("WARN:  {}", summary.warn);
                println!("ERROR: {}", summary.error);
                println!("DEBUG: {}", summary.debug);
                println!("Error rate: {:.2}%", summary.error_rate);
                println!("Warning rate: {:.2}%", summary.warning_rate);
            }

            eprintln!("Skipped malformed lines: {invalid}");
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
