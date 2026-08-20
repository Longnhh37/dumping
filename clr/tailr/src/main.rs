use anyhow::{Result, anyhow};
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = true)]
    paths: Vec<String>,

    #[arg(short('n'), long, conflicts_with = "bytes", allow_hyphen_values = true)]
    lines: Option<String>,

    #[arg(short('c'), long, conflicts_with = "lines", allow_hyphen_values = true)]
    bytes: Option<String>,

    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Last(usize),
    FromStart(usize),
}

#[derive(Debug, Copy, Clone)]
enum TextUnit {
    Bytes,
    Lines,
}

// --------------------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------------------
fn run(args: Args) -> Result<()> {
    let multiple = args.paths.len() > 1;

    let (mode, text_unit) = if let Some(s) = args.bytes {
        (parse_mode(&s, "byte")?, TextUnit::Bytes)
    } else if let Some(s) = args.lines {
        (parse_mode(&s, "line")?, TextUnit::Lines)
    } else {
        (Mode::Last(10), TextUnit::Lines)
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();

    for (i, path) in args.paths.iter().enumerate() {
        let file = match File::open(path) {
            Err(e) => {
                eprintln!("{path}: {e}");
                continue;
            }
            Ok(f) => f,
        };

        if !args.quiet && multiple {
            if i > 0 {
                writeln!(out)?;
            }
            writeln!(out, "==> {path} <==")?;
        }

        process(file, mode, text_unit)?;
    }

    Ok(())
}

// --------------------------------------------------------------
fn parse_mode(s: &str, unit: &str) -> Result<Mode> {
    let err = || anyhow!("illegal {} count -- {}", unit, s);

    if let Some(r) = s.strip_prefix('+') {
        Ok(Mode::FromStart(r.parse().map_err(|_| err())?))
    } else {
        let n = s
            .strip_prefix('-')
            .unwrap_or(s)
            .parse()
            .map_err(|_| err())?;
        Ok(Mode::Last(n))
    }
}

// --------------------------------------------------------------
fn read_to_string_lossy(mut r: impl Read) -> Result<String> {
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).into_owned())
}

// --------------------------------------------------------------
fn split_lines(content: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = content
        .split('\n')
        .collect();

    if lines.last() == Some(&"") {
        lines.pop();
    }
    lines
}

// --------------------------------------------------------------
fn process(file: File, mode: Mode, text_unit: TextUnit) -> Result<()> {
    if let Mode::Last(0) = mode {
        return Ok(());
    }

    match (mode, text_unit) {
        (Mode::Last(n), TextUnit::Bytes) => bytes_from_last(file, n),
        (Mode::Last(n), TextUnit::Lines) => lines_from_last(file, n),
        (Mode::FromStart(n), TextUnit::Bytes) => bytes_from_start(file, n),
        (Mode::FromStart(n), TextUnit::Lines) => lines_from_start(file, n),
    }
}

// --------------------------------------------------------------
fn lines_from_last(file: File, n: usize) -> Result<()> {
    let content = read_to_string_lossy(file)?;
    let lines = split_lines(&content);
    let start = lines.len().saturating_sub(n);
    let mut out = io::stdout().lock();

    for line in &lines[start..] {
        writeln!(out, "{}", line)?;
    }

    Ok(())
}

// --------------------------------------------------------------
fn lines_from_start(file: File, n: usize) -> Result<()> {
    let content = read_to_string_lossy(file)?;
    let lines = split_lines(&content);
    let mut out = std::io::stdout();

    for line in lines.iter().skip(n.saturating_sub(1)) {
        writeln!(out, "{}", line)?;
    }

    Ok(())
}

// --------------------------------------------------------------
fn bytes_from_start(mut file: File, n: usize) -> Result<()> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let skip = n.saturating_sub(1).min(buf.len());
    write!(
        std::io::stdout(),
        "{}",
        String::from_utf8_lossy(&buf[skip..])
    )?;

    Ok(())
}

// --------------------------------------------------------------
fn bytes_from_last(mut file: File, n: usize) -> Result<()> {
    let len = file.metadata()?.len();
    file.seek(SeekFrom::Start(len.saturating_sub(n as u64)))?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    write!(std::io::stdout(), "{}", String::from_utf8_lossy(&buf))?;

    Ok(())
}
