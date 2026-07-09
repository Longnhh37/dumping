use anyhow::{Result, anyhow};
use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use colorize::AnsiColor;

// -- Args parsing -----------------------------------------------

#[derive(Parser, Debug)]
#[command(name = "calr", version, about)]
struct Args {
    #[arg()]
    year: Option<u32>,
    #[arg(short = 'y', long = "year", conflicts_with_all = ["year", "month"])]
    whole_year: bool,
    #[arg(short = 'm', value_name = "MONTH")]
    month: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct Date {
    year: u32,
    month: Month,
}

#[derive(Debug, Clone, Copy)]
enum Month {
    All,
    One(u8),
}

impl Args {
    fn validate(self) -> Result<Date> {
        let now = Local::now();
        let cur_year = now.year() as u32;
        let cur_month = now.month() as u8;

        if self.whole_year {
            return Ok(Date {
                year: cur_year,
                month: Month::All,
            });
        }

        if let Some(raw_month) = self.month {
            let month = parse_month(&raw_month)?;
            return Ok(Date {
                year: self.year.unwrap_or(cur_year),
                month: Month::One(month),
            });
        }

        if let Some(y) = self.year {
            if !(1..=9999).contains(&y) {
                return Err(anyhow!(
                    "error: invalid value '{y}' for '[YEAR]': {y} is not in 1..=9999"
                ));
            }
            return Ok(Date {
                year: y,
                month: Month::All,
            });
        }

        Ok(Date {
            year: cur_year,
            month: Month::One(cur_month),
        })
    }
}

fn parse_month(s: &str) -> Result<u8> {
    let m = s.to_ascii_lowercase();
    let month = match m.as_str() {
        "ja" | "jan" | "january" => 1,
        "f" | "feb" | "february" => 2,
        "mar" | "march" => 3,
        "ap" | "apr" | "april" => 4,
        "may" => 5,
        "jun" | "june" => 6,
        "jul" | "july" => 7,
        "au" | "aug" | "august" => 8,
        "s" | "sep" | "september" => 9,
        "o" | "oct" | "october" => 10,
        "n" | "nov" | "november" => 11,
        "d" | "dec" | "december" => 12,
        other => match other.parse::<u8>() {
            Ok(n @ 1..=12) => n,
            Ok(n) => return Err(anyhow!(r#"month "{n}" not in the range 1 through 12"#)),
            Err(_) => return Err(anyhow!(r#"Invalid month "{}""#, s)),
        },
    };
    Ok(month)
}

// -- Generate calendar ---------------------------

struct CalendarMonth {
    month: u8,
    year: u32,
    weeks: Vec<Vec<Option<u8>>>,
}

fn generate_calendar(date: Date) -> Vec<CalendarMonth> {
    match date.month {
        Month::One(m) => vec![generate_month(m, date.year)],
        Month::All => (1u8..=12).map(|m| generate_month(m, date.year)).collect(),
    }
}

fn generate_month(month: u8, year: u32) -> CalendarMonth {
    let first = NaiveDate::from_ymd_opt(year as i32, month as u32, 1).expect("valid date");

    let next_first = if month == 12 {
        NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year as i32, month as u32 + 1, 1)
    }
    .expect("valid date");

    let days_in_month = next_first.signed_duration_since(first).num_days() as u8;
    let start_col = first.weekday().num_days_from_sunday() as usize; // Sun=0…Sat=6

    let mut weeks: Vec<Vec<Option<u8>>> = Vec::new();
    let mut week: Vec<Option<u8>> = vec![None; start_col];

    for day in 1..=days_in_month {
        week.push(Some(day));
        if week.len() == 7 {
            weeks.push(std::mem::take(&mut week));
        }
    }
    if !week.is_empty() {
        week.resize(7, None);
        weeks.push(week);
    }

    CalendarMonth { month, year, weeks }
}

// -- Style helpers -----------------------------------------------

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Visible column width  =  "Su Mo Tu We Th Fr Sa".len()  =  20
const COL_WIDTH: usize = 20;

/// Strip ANSI escape sequences (`ESC [ … m`) so we count only printable chars.
/// Handles multi-param codes such as `\x1b[1;31m`.
fn visible_len(s: &str) -> usize {
    let mut len = 0usize;
    let mut in_esc = false;
    for c in s.chars() {
        match (in_esc, c) {
            (false, '\x1b') => in_esc = true,
            (true, 'm') => in_esc = false,
            (false, _) => len += 1,
            _ => {} // skip escape-sequence body chars
        }
    }
    len
}

/// Right-pad `s` to `width` *visible* characters (ANSI-escape–aware).
fn pad_to(s: &str, width: usize) -> String {
    let vlen = visible_len(s);
    if vlen < width {
        format!("{}{}", s, " ".repeat(width - vlen))
    } else {
        s.to_string()
    }
}

// ── Palette ─────────────────────────────────────────────────────────
// All colour decisions live here — one place to tweak the whole theme.

/// "   April 2026   "  →  bold bright-yellow
fn style_title(plain: &str) -> String {
    plain.to_string().bold().b_yellow()
}

/// Column header cell.  col 0 = Sun, col 6 = Sat  (weekend = red).
fn style_header_cell(label: &str, col: usize) -> String {
    if col == 0 || col == 6 {
        label.to_string().bold().b_red()
    } else {
        label.to_string().bold().cyan()
    }
}

/// Weekend day number (Su / Sa column).
fn style_weekend(s: &str) -> String {
    s.to_string().b_red()
}

/// Today's day number — most prominent element on screen.
fn style_today(s: &str) -> String {
    s.to_string().bold().blue()
}

/// Year banner at the top of `-y` / full-year output.
fn style_year_banner(plain: &str) -> String {
    plain.to_string().bold().b_cyan()
}

// -- Render calendar ---------------------------

/// Build 8 fixed-height lines for one month (all ANSI-aware).
/// Pass `today = Some((year, month, day))` to highlight the current day.
fn month_lines(cal: &CalendarMonth, today: Option<(u32, u8, u8)>) -> Vec<String> {
    // ── Title ────────────────────────────────────────────────────
    let title_plain = MONTH_NAMES[cal.month as usize - 1].to_string();
    // Centre the *plain* string first (simple arithmetic), then colourize.
    let title_line = style_title(&format!("{:^width$}", title_plain, width = COL_WIDTH));

    // ── Day-of-week header ────────────────────────────────────────
    let header_labels = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
    let header_line: String = header_labels
        .iter()
        .enumerate()
        .map(|(col, lbl)| style_header_cell(lbl, col))
        .collect::<Vec<_>>()
        .join(" ");

    let mut lines = vec![title_line, header_line];

    // ── Week rows ─────────────────────────────────────────────────
    for week in &cal.weeks {
        let row: String = week
            .iter()
            .enumerate()
            .map(|(col, slot)| match slot {
                None => "  ".to_string(),
                Some(n) => {
                    let s = format!("{:2}", n);
                    let is_today = today
                        .map(|(ty, tm, td)| ty == cal.year && tm == cal.month && *n == td)
                        .unwrap_or(false);
                    let is_weekend = col == 0 || col == 6;

                    if is_today {
                        style_today(&s)
                    } else if is_weekend {
                        style_weekend(&s)
                    } else {
                        s
                    } // plain — no colour noise on weekdays
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        lines.push(row);
    }

    // ── Normalise height to 8 lines (title + header + ≤6 week rows) ──
    while lines.len() < 8 {
        lines.push(" ".repeat(COL_WIDTH));
    }
    lines
}

fn render_month(cal: &CalendarMonth) -> String {
    let now = Local::now();
    let today = Some((now.year() as u32, now.month() as u8, now.day() as u8));
    month_lines(cal, today).join("\n")
}

/// Render all 12 months in a 3-column grid, Unix `cal -y` style.
fn render_year(cals: &[CalendarMonth]) -> String {
    let now = Local::now();
    let today = Some((now.year() as u32, now.month() as u8, now.day() as u8));

    // 3×COL_WIDTH + 2×2 gutter = 64 visible chars
    let banner_plain = format!("{:^64}", cals[0].year);
    let mut out = format!("{}\n\n", style_year_banner(&banner_plain));

    for chunk in cals.chunks(3) {
        let rendered: Vec<Vec<String>> = chunk.iter().map(|c| month_lines(c, today)).collect();

        let max_lines = rendered.iter().map(|r| r.len()).max().unwrap_or(0);

        for i in 0..max_lines {
            let row = rendered
                .iter()
                .map(|r| {
                    let s = r.get(i).cloned().unwrap_or_default();
                    pad_to(&s, COL_WIDTH) // ANSI-aware — columns stay straight
                })
                .collect::<Vec<_>>()
                .join("  "); // 2-space gutter between months

            out.push_str(row.trim_end());
            out.push('\n');
        }
        out.push('\n');
    }
    out
}

// -- Business logic ---------------------------------------------------

fn run(args: Args) -> Result<()> {
    let date = args.validate()?;
    let cal = generate_calendar(date);
    match date.month {
        Month::One(_) => println!("{}", render_month(&cal[0])),
        Month::All => print!("{}", render_year(&cal)),
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
