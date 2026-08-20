use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    files: Vec<String>,
    #[arg(short = 'c', long = "count")]
    show_count: bool,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    // ===== Input =====
    let mut reader: Box<dyn BufRead> = match args.files.first().map(String::as_str) {
        Some("-") | None    => Box::new(io::stdin().lock()),
        Some(path)          => {
            let file = File::open(path).map_err(|e| anyhow::anyhow!("{}: {}", path, e))?;
            Box::new(BufReader::new(file))
        }
    };

    // ===== Output =====
    let mut writer: Box<dyn Write> = match args.files.get(1) {
        Some(path)  => Box::new(BufWriter::new(File::create(path)?)),
        None        => Box::new(BufWriter::new(io::stdout().lock())),
    };

    // ===== Core =====

    let mut prev = String::new();
    let mut cur = String::new();
    let mut count = 0usize;
    let _ = count;

    if reader.read_line(&mut prev)? == 0 {
        return Ok(());
    }
    count = 1;

    loop {
        cur.clear();
        if reader.read_line(&mut cur)? == 0 {
            break;
        }
        if cur.strip_suffix('\n').unwrap_or(&cur) == prev.strip_suffix('\n').unwrap_or(&prev) {
            count += 1;
        } else {
            emit(&mut writer, count, &prev, args.show_count)?;
            std::mem::swap(&mut prev, &mut cur);
            count = 1;
        }
    }

    if count > 0 {
        emit(&mut writer, count, &prev, args.show_count)?;
    }

    Ok(())
}

fn emit<W: Write>(w: &mut W, count: usize, line: &str, show: bool) -> io::Result<()> {
    if show {
        write!(w, "{:>4} {}", count, line)
    } else {
        write!(w, "{}", line)
    }
}
