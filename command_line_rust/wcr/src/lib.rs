use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::ops::Add;

type PrgResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(name = "wcr", version, about = "minimal rust wc")]
struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(short, long = "bytes", conflicts_with = "m")]
    c: bool,

    #[arg(short, long = "lines")]
    l: bool,

    #[arg(short, long = "words")]
    w: bool,

    #[arg(short, long  = "chars", conflicts_with = "c")]
    m: bool,
}

#[derive(Debug, Clone, Copy)]
enum TextUnit {
    Bytes,
    Chars,
}

#[derive(Debug, Clone, Copy)]
struct Mode {
    text_unit: TextUnit,
    show_lines: bool,
    show_words: bool,
    show_text: bool,
}

impl Mode {
    fn from_args(args: &Args) -> Self {
        let no_flags = !args.c && !args.l && !args.w && !args.m;

        if no_flags {
            return Self {
                text_unit: TextUnit::Bytes,
                show_lines: true,
                show_words: true,
                show_text: true,
            };
        }

        let text_unit = if args.c {
            TextUnit::Bytes
        } else if args.m {
            TextUnit::Chars
        } else {
            TextUnit::Bytes
        };

        let show_text = args.c || args.m;

        Self {
            text_unit,
            show_lines: args.l,
            show_words: args.w,
            show_text,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Counter {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

impl Add for Counter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            bytes: self.bytes + rhs.bytes,
            chars: self.chars + rhs.chars,
        }
    }
}

pub fn run() -> PrgResult<()> {
    let args = Args::parse();
    let mode = Mode::from_args(&args);

    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut total = Counter::default();
    let multiple = args.files.len() > 1;

    for file in &args.files {
        let counter = match dispatch_input(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}: {}", file, e);
                continue;
            }
        };

        total = total + counter;

        print_counter(&mut out, file, counter, &mode)?;
    }

    if multiple {
        print_counter(&mut out, "total", total, &mode)?;
    }

    Ok(())
}

fn dispatch_input(path: &str) -> PrgResult<Counter> {
    if path == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_input(reader)
    } else {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        process_input(reader)
    }
}

fn process_input<R: BufRead>(mut reader: R) -> PrgResult<Counter> {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;

    let mut in_word = false;
    let mut leftover: Vec<u8> = Vec::new();

    loop {
        let buf = reader.fill_buf()?;
        let buflen = buf.len();
        if buf.is_empty() {
            break;
        }

        // Bytes
        bytes += buflen;

        // Lines + words
        for &b in buf {
            if b == b'\n' {
                lines += 1;
            }
            if b.is_ascii_whitespace() {
                in_word = false;
            } else {
                if !in_word {
                    words += 1;
                    in_word = true;
                }
            }
        }

        // Chars (utf8 streaming)
        if leftover.is_empty() {
            match std::str::from_utf8(buf) {
                Ok(s) => chars += s.chars().count(),
                Err(e) => {
                    let valid = e.valid_up_to();

                    if valid > 0 {
                        let s = unsafe { std::str::from_utf8_unchecked(&buf[..valid]) };
                        chars += s.chars().count();
                    }

                    leftover.extend_from_slice(&buf[valid..]);
                }
            }
        } else {
            let mut combined = Vec::with_capacity(leftover.len() + buflen);
            combined.extend_from_slice(&leftover);
            combined.extend_from_slice(buf);

            match std::str::from_utf8(&combined) {
                Ok(s) => {
                    chars += s.chars().count();
                    leftover.clear();
                }
                Err(e) => {
                    let valid = e.valid_up_to();

                    if valid > 0 {
                        let s = unsafe { std::str::from_utf8_unchecked(&combined[..valid]) };
                        chars += s.chars().count();
                    }

                    leftover.clear();
                    leftover.extend_from_slice(&buf[valid..]);
                }
            }
        }

        reader.consume(buflen);
    }

    Ok(Counter {
        lines,
        words,
        bytes,
        chars,
    })
}

fn print_counter(out: &mut impl Write, file: &str, c: Counter, mode: &Mode) -> PrgResult<()> {
    if mode.show_lines {
        write!(out, "{:>8}", c.lines)?;
    }
    if mode.show_words {
        write!(out, "{:>8}", c.words)?;
    }
    if mode.show_text {
        match mode.text_unit {
            TextUnit::Bytes => write!(out, "{:>8}", c.bytes)?,
            TextUnit::Chars => write!(out, "{:>8}", c.chars)?,
        }
    }
    if file != "-" {
        write!(out, " {}", file)?;
    }

    writeln!(out)?;
    Ok(())
}
