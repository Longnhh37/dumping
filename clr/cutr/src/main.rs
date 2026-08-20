use anyhow::{Result, anyhow, bail};
use clap::Parser;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    ops::Range,
};

#[derive(Parser, Debug)]
#[command(name = "cutr", author, version, about)]
struct Args {
    #[arg(value_name = "PATH", default_value = "-")]
    files: Vec<String>,

    #[arg(short('d'), long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,

    #[command(flatten)]
    extract: ArgsExtract,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
    #[arg(short = 'f', long, value_name = "FIELDS")]
    fields: Option<String>,

    #[arg(short('b'), long, value_name = "BYTES")]
    bytes: Option<String>,

    #[arg(short('c'), long, value_name = "CHARS")]
    chars: Option<String>,
}

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

// -------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// -------------------------------------------------
fn run(args: Args) -> Result<()> {
    let delim_bytes = args.delimiter.as_bytes();
    if delim_bytes.len() != 1 {
        bail!(r#"--delim "{}" must be a single byte"#, args.delimiter);
    }
    let delimiter: u8 = *delim_bytes.first().unwrap();

    let extract = if let Some(fields) = args.extract.fields.map(parse_pos).transpose()? {
        Extract::Fields(fields)
    } else if let Some(bytes) = args.extract.bytes.map(parse_pos).transpose()? {
        Extract::Bytes(bytes)
    } else if let Some(chars) = args.extract.chars.map(parse_pos).transpose()? {
        Extract::Chars(chars)
    } else {
        unreachable!("Must have --fields, --bytes, or --chars");
    };

    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => match &extract {
                Extract::Fields(field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(delimiter)
                        .has_headers(false)
                        .from_reader(file);

                    let mut writer = WriterBuilder::new()
                        .delimiter(delimiter)
                        .from_writer(io::stdout().lock());

                    for record in reader.records() {
                        writer.write_record(extract_fields(
                            &record?, field_pos
                        ))?;
                    }
                }
                Extract::Bytes(byte_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_bytes(&line?, byte_pos));
                    }
                }
                Extract::Chars(char_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_chars(&line?, char_pos));
                    }
                }
            },
        }
    }

    Ok(())
}

// -------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// -------------------------------------------------
fn parse_index(input: &str) -> Result<usize> {
    let value_error = || anyhow!(r#"illegal list value: "{input}""#);
    if input.starts_with('+') {
        Err(value_error())
    } else {
        input
            .parse::<NonZeroUsize>()
            .map(|n| usize::from(n) - 1)
            .map_err(|_| value_error())
    }
}

// -------------------------------------------------
fn parse_pos(range: String) -> Result<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(',')
        .map(|v| {
            parse_index(v).map(|n| n..n + 1).or_else(|e| {
                range_re.captures(v).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        bail!(
                            "First number in range ({}) \
                        must be lower than second number ({})",
                            n1 + 1,
                            n2 + 1
                        );
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
}

// -------------------------------------------------
fn extract_fields<'a>(record: &'a StringRecord, field_pos: &[Range<usize>]) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

// -------------------------------------------------
fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i).copied()))
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

// -------------------------------------------------
fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    char_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i).copied()))
        .collect()
}
