use crate::parser::error::ParseError;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Literal(String),
    WordSep,

    VarExpand(String), // $FOO or ${FOO}
    LastStatus,        // $?

    Pipe,              // |
    RedirectOut,       // > / 1>
    RedirectIn,        // <
    RedirectAppend,    // >> / 1>>
    RedirectErr,       // 2>
    RedirectErrAppend, // 2>>
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut chars = input.chars().peekable();

    let mut in_single = false;
    let mut in_double = false;
    let mut escape = false;

    macro_rules! flush_buf {
        () => {
            if !buf.is_empty() {
                tokens.push(Token::Literal(buf.clone()));
                buf.clear();
            }
        };
    }

    while let Some(c) = chars.next() {
        if escape {
            buf.push(c);
            escape = false;
            continue;
        }

        match c {
            '\\' if !in_single => escape = true,

            '\'' if !in_double => in_single = !in_single,

            '"' if !in_single => in_double = !in_double,

            // --- $ expansion --- active outside single quote -----------------------
            '$' if !in_single => {
                flush_buf!();

                match chars.peek().copied() {
                    // $?
                    Some('?') => {
                        chars.next(); // consume '?'
                        tokens.push(Token::LastStatus);
                    }

                    // ${VARNAME}
                    Some('{') => {
                        chars.next(); // consume '{'
                        let name: String = chars.by_ref().take_while(|&c| c != '}').collect();
                        tokens.push(Token::VarExpand(name));
                    }

                    // $VARNAME
                    Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                        let mut name = String::new();

                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_alphanumeric() || c == '_' {
                                name.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        tokens.push(Token::VarExpand(name));
                    }

                    // bare $ -> literal character
                    _ => buf.push('$'),
                }
            }

            // --- 1> / 1>> --------------------------------------------------------
            '1' if !in_single && !in_double && buf.is_empty() => {
                if chars.peek() == Some(&'>') {
                    chars.next();
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(Token::RedirectAppend);
                    } else {
                        tokens.push(Token::RedirectOut);
                    }
                } else {
                    // ordinary '1' character
                    buf.push('1');
                }
            }
            // --- 2> / 2>> --------------------------------------------------------
            '2' if !in_single && !in_double && buf.is_empty() => {
                if chars.peek() == Some(&'>') {
                    chars.next();
                    if chars.peek() == Some(&'>') {
                        chars.next();
                        tokens.push(Token::RedirectErrAppend);
                    } else {
                        tokens.push(Token::RedirectErr);
                    }
                } else {
                    // ordinary '2' character
                    buf.push('2');
                }
            }

            // --- operators (suppressed inside quotes) ----------------------------
            '|' if !in_single && !in_double => {
                flush_buf!();
                tokens.push(Token::Pipe);
            }

            '>' if !in_single && !in_double => {
                flush_buf!();

                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::RedirectAppend);
                } else {
                    tokens.push(Token::RedirectOut);
                }
            }

            '<' if !in_single && !in_double => {
                flush_buf!();
                tokens.push(Token::RedirectIn);
            }

            c if c.is_whitespace() && !in_single && !in_double => {
                let had_content = !buf.is_empty();
                flush_buf!();

                let last_is_word = matches!(
                    tokens.last(),
                    Some(Token::Literal(_)) | Some(Token::VarExpand(_)) | Some(Token::LastStatus)
                );
                if had_content || last_is_word {
                    tokens.push(Token::WordSep);
                }
            }

            _ => buf.push(c),
        }
    }

    if escape {
        return Err(ParseError::TrailingEscape);
    }
    if in_single {
        return Err(ParseError::UnterminatedSingleQuote);
    }
    if in_double {
        return Err(ParseError::UnterminatedDoubleQuote);
    }

    flush_buf!();

    Ok(tokens)
}
