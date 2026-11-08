// parsing outside data is the ONLY way in.
mod instruction {
    use std::{error::Error, fmt};

    // --- Errors ---
    #[derive(Debug, PartialEq, Eq)]
    pub enum ParseError {
        WrongFieldCount { expected: usize, actual: usize },
        UnknownKind(String),
        UnknownCurrency(String),
        BadIban(&'static str),
        BadAmount(&'static str),
        BadTxId,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::WrongFieldCount { expected, actual } => {
                    write!(f, "expected {expected} fields, got {actual}")
                }
                Self::UnknownKind(k) => write!(f, "unknown instruction kind: {k}"),
                Self::UnknownCurrency(c) => write!(f, "unsupported currency: {c}"),
                Self::BadIban(why) => write!(f, "invalid IBAN: {why}"),
                Self::BadAmount(why) => write!(f, "invalid amount: {why}"),
                Self::BadTxId => write!(f, "invalid transaction ID"),
            }
        }
    }

    impl Error for ParseError {}

    // --- Parsed type 1: Iban (flat string, checksum) ---
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Iban(String); // private field: only constructible via FromStr

    impl Iban {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl std::str::FromStr for Iban {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Normalize: drop whitespace, uppercase
            let iban: String = s
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_ascii_uppercase())
                .collect();
            let b = iban.as_bytes();

            if !(15..=34).contains(&b.len()) {
                return Err(ParseError::BadIban("length must be between 15 to 34"));
            }
            if !b.iter().all(u8::is_ascii_alphanumeric) {
                return Err(ParseError::BadIban("only A-Z and 0-9 allowed"));
            }
            if !b[..2].iter().all(u8::is_ascii_uppercase) || !b[2..4].iter().all(u8::is_ascii_digit)
            {
                return Err(ParseError::BadIban("needs 2 letters + 2 check digits"));
            }

            // ISO 7064 mod-97: rotate the first 4 chars to the end, map letters to 10..=35
            let mut rem: u32 = 0;
            for &c in b[4..].iter().chain(&b[..4]) {
                rem = match c {
                    b'0'..=b'9' => (rem * 10 + u32::from(c - b'0')) % 97,
                    _ => (rem * 100 + u32::from(c - b'A') + 10) % 97,
                };
            }
            if rem != 1 {
                return Err(ParseError::BadIban("checksum mismatch"));
            }

            Ok(Iban(iban))
        }
    }

    // --- Parsed type 2: Currency ---
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Currency {
        Sgd,
        Usd,
        Eur,
    }

    impl std::str::FromStr for Currency {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "SGD" => Ok(Self::Sgd),
                "USD" => Ok(Self::Usd),
                "EUR" => Ok(Self::Eur),
                other => Err(ParseError::UnknownCurrency(other.to_string())),
            }
        }
    }

    // --- Parsed type 3: Money (integer minor units, never f64) ---
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Money {
        minor_units: u64,
        currency: Currency,
    }

    impl Money {
        const MAX_MINOR: u64 = 1_000_000_000_00; // 1 billion

        pub fn minor_units(&self) -> u64 {
            self.minor_units
        }

        pub fn currency(&self) -> Currency {
            self.currency
        }

        fn parse(amount: &str, currency: Currency) -> Result<Self, ParseError> {
            // Exactly "<digits>.<2 digits>", so no "+5", "1e3", "10.5", or "-3.00"
            let (whole, frac) = amount
                .split_once('.')
                .ok_or(ParseError::BadAmount("expected <digits>.<2 digits>"))?;
            if !all_digits(whole) || frac.len() != 2 || !all_digits(frac) {
                return Err(ParseError::BadAmount("expected <digits>.<2 digits>"));
            }

            let too_large = || ParseError::BadAmount("too large");
            let whole: u64 = whole.parse().map_err(|_| too_large())?;
            let frac: u64 = frac.parse().expect("2 ASCII digits always parse");
            let minor_units = whole
                .checked_mul(100)
                .and_then(|w| w.checked_add(frac))
                .ok_or_else(too_large)?;

            if minor_units == 0 {
                return Err(ParseError::BadAmount("must be positive"));
            }
            if minor_units > Self::MAX_MINOR {
                return Err(ParseError::BadAmount("exceeds limit"));
            }

            Ok(Money {
                minor_units,
                currency,
            })
        }
    }

    fn all_digits(s: &str) -> bool {
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
    }

    // --- Composed + polymorphic validated types ---
    #[derive(Debug)]
    pub struct Payment {
        pub to: Iban,
        pub money: Money,
    }

    #[derive(Debug)]
    pub struct Refund {
        pub original_tx: u64,
        pub money: Money,
    }

    #[derive(Debug)]
    pub enum Instruction {
        Pay(Payment),
        Refund(Refund),
    }

    /// Unstrusted input straight from a file/socket
    pub struct RawLine<'a>(pub &'a str);

    // The ONE boundary: "KIND | target | amount | currency"
    impl TryFrom<RawLine<'_>> for Instruction {
        type Error = ParseError;

        fn try_from(raw: RawLine<'_>) -> Result<Self, Self::Error> {
            let fields: Vec<&str> = raw.0.split('|').map(str::trim).collect();
            let [kind, target, amount, currency] = fields.as_slice() else {
                return Err(ParseError::WrongFieldCount {
                    expected: 4,
                    actual: fields.len(),
                });
            };

            let money = Money::parse(amount, currency.parse()?)?;

            match *kind {
                "PAY" => Ok(Instruction::Pay(Payment {
                    to: target.parse()?,
                    money,
                })),
                "REFUND" => {
                    if !all_digits(target) {
                        return Err(ParseError::BadTxId);
                    }
                    let original_tx = target.parse().map_err(|_| ParseError::BadTxId)?;
                    Ok(Instruction::Refund(Refund { original_tx, money }))
                }
                other => Err(ParseError::UnknownKind(other.to_owned())),
            }
        }
    }
}

use instruction::{Iban, Instruction, Money, RawLine};

// --- Downstream code: takes parsed types, never re-validates ---

/// Slicing is safe: an `Iban` is always ASCII and at least 15 bytes long.
fn mask_iban(iban: &Iban) -> String {
    let s = iban.as_str();
    format!("{}...{}", &s[..4], &s[s.len() - 4..])
}

fn format_money(m: &Money) -> String {
    let (w, f) = (m.minor_units() / 100, m.minor_units() % 100);
    format!("{w}.{f:02} {:?}", m.currency())
}

/// Needs manual review? One rule, one place, no input checks.
fn needs_review(m: &Money) -> bool {
    m.minor_units() >= 10_000_00
}

fn describe(i: &Instruction) -> String {
    match i {
        Instruction::Pay(p) => format!("PAY {} to {}", format_money(&p.money), mask_iban(&p.to)),
        Instruction::Refund(r) => format!(
            "REFUND {} for tx #{}",
            format_money(&r.money),
            r.original_tx
        ),
    }
}

fn main() {
    let lines = [
        "PAY | DE89 3704 0044 0532 0130 00 | 250.00 | EUR",
        "PAY | GB82 WEST 1234 5698 7654 32 | 15000.00 | USD",
        "REFUND | 1001 | 12.50 | SGD",
        "PAY | DE89 3704 0044 0532 0130 01 | 250.00 | EUR", // bad checksum
        "PAY | GB82 WEST 1234 5698 7654 32 | 10.5 | USD",   // bad amount
        "PAY | GB82 WEST 1234 5698 7654 32 | 5.00 | GBP",   // unsupported currency
        "TRANSFER | x | 1.00 | SGD",                        // unknown kind
        "PAY | only | three",                               // wrong field count
    ];

    for (n, line) in lines.iter().enumerate() {
        // Boundary: &str -> Instruction. After this line, nothing is re-checked
        match Instruction::try_from(RawLine(line)) {
            Ok(ins) => {
                let flag = match &ins {
                    Instruction::Pay(p) => needs_review(&p.money),
                    Instruction::Refund(r) => needs_review(&r.money),
                };
                println!(
                    "#{n} ok   : {}{}",
                    describe(&ins),
                    if flag { "  [REVIEW]" } else { "" }
                );
            }
            Err(e) => eprintln!("#{n} error: {e}"),
        }
    }

    // Forging a validated value does not compile:
    // let fake = Iban("NOT-AN-IBAN".into());
    //                  ^^^^^^^^^^^^^^^^^^ ERROR: field `0` of `Iban` is private
}
