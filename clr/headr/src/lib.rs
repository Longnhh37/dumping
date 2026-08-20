use anyhow::{Result, bail};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// ============================
// CLI
// ============================

#[derive(Parser, Debug)]
#[command(name = "headr", version, about = "Minimal rust head")]
struct Args {
    /// Files to read, use '-' for stdin
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long = "lines", conflicts_with = "bytes")]
    lines: Option<usize>,

    #[arg(short = 'c', long = "bytes", conflicts_with = "lines")]
    bytes: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Limit {
    Lines(usize),
    Bytes(usize),
}

// ============================
// parse args
// ============================

fn get_args() -> Result<(Vec<String>, Limit)> {
    let args = Args::parse();

    // Parse errors → bail! → stop process immediately (correct)
    let limit = match (args.lines, args.bytes) {
        (Some(0), _) => bail!("illegal line count -- 0"),
        (_, Some(0)) => bail!("illegal byte count -- 0"),
        (_, Some(n)) => Limit::Bytes(n),
        (Some(n), _) => Limit::Lines(n),
        _ => Limit::Lines(10),
    };

    Ok((args.files, limit))
}

// ============================
// run
// ============================

pub fn run() -> Result<()> {
    // Parse errors propagate up → process stops
    let (files, limit) = get_args()?;
    let multiple = files.len() > 1;

    // Lock stdout once for the whole run (more efficient)
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for (i, file) in files.iter().enumerate() {
        if multiple {
            if i > 0 {
                // Write error on stdout → stop immediately (broken pipe, etc.)
                writeln!(out)?;
            }
            let label = if file == "-" { "stdin" } else { file.as_str() };
            writeln!(out, "==> {label} <==")?;
        }

        // IO read/open errors → eprintln, continue to next file
        if let Err(e) = process_file(file, limit, &mut out) {
            let label = if file == "-" { "stdin".to_owned() } else { file.clone() };
            eprintln!("{label}: {e}");
        }
    }

    Ok(())
}

// ============================
// per-file dispatch
// ============================

fn process_file(file: &str, limit: Limit, out: &mut impl Write) -> Result<()> {
    if file == "-" {
        process(io::stdin().lock(), limit, out)
    } else {
        // File::open error → propagates → caught in run() → eprintln, continue
        process(BufReader::new(File::open(file)?), limit, out)
    }
}

// ============================
// core processing
// ============================

fn process<R: BufRead>(reader: R, limit: Limit, out: &mut impl Write) -> Result<()> {
    match limit {
        Limit::Lines(n) => process_lines(reader, n, out),
        Limit::Bytes(n) => process_bytes(reader, n, out),
    }
}

// ---------- lines ----------
fn process_lines<R: BufRead>(mut reader: R, mut remaining: usize, out: &mut impl Write) -> Result<()> {
    let mut buf = String::new();

    while remaining > 0 {
        buf.clear();
        // read_line error → ? → propagates → eprintln, continue next file
        if reader.read_line(&mut buf)? == 0 {
            break; // EOF
        }
        // write error → ? → propagates up past run()'s per-file catch → stop process
        out.write_all(buf.as_bytes())?;
        remaining -= 1;
    }

    Ok(())
}

// ---------- bytes ----------
fn process_bytes<R: BufRead>(mut reader: R, mut remaining: usize, out: &mut impl Write) -> Result<()> {
    while remaining > 0 {
        // fill_buf error → ? → propagates → eprintln, continue next file
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break; // EOF
        }
        let n = remaining.min(buf.len());
        // write error → ? → propagates up past run()'s per-file catch → stop process
        out.write_all(&buf[..n])?;
        reader.consume(n);
        remaining -= n;
    }

    Ok(())
}
