use crate::{
    error::{RloxErr, scan_error::ScanError},
    lexer::{literal::Literal, token::Token, token_type::TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into().chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, RloxErr> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        Ok(self.tokens)
    }

    // ====================================================================================
    // internal processing
    // ====================================================================================

    fn scan_token(&mut self) -> Result<(), RloxErr> {
        use TokenType::*;

        let cur = match self.advance() {
            Some(ch) => ch,
            None => return Ok(()),
        };

        match cur {
            '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | ';' | '*' => {
                let tok = match_token(cur).unwrap();
                self.add_token(tok);
            }

            '!' => {
                if self.match_char('=') {
                    self.add_token(BangEqual);
                } else {
                    self.add_token(Bang);
                }
            }

            '=' => {
                if self.match_char('=') {
                    self.add_token(EqualEqual);
                } else {
                    self.add_token(Equal);
                }
            }

            '<' => {
                if self.match_char('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }

            '>' => {
                if self.match_char('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }

            '/' => {
                if self.peek() == Some('/') {
                    while !self.is_at_end() && self.peek().unwrap() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.scan_string()?,
            '0'..='9' => self.scan_number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier()?,
            _ => {
                return Err(ScanError::UnexpectedCharacter {
                    line: self.line,
                    ch: cur,
                }
                .into());
            }
        }

        Ok(())
    }

    fn scan_string(&mut self) -> Result<(), RloxErr> {
        let mut found_closed_quote = false;

        while let Some(ch) = self.peek() {
            if ch == '\n' {
                self.line += 1;
            }
            if ch == '"' {
                found_closed_quote = true;
                break;
            }
            self.advance();
        }

        if !found_closed_quote {
            return Err(ScanError::UnterminatedString { line: self.line }.into());
        }

        self.advance();
        let literal = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token_with_literal(TokenType::String, Literal::String(literal));
        Ok(())
    }

    fn scan_number(&mut self) -> Result<(), RloxErr> {
        while let Some(ch) = self.peek()
            && ch.is_ascii_digit()
        {
            self.advance();
        }

        if self.peek() == Some('.')
            && let Some(ch) = self.peek_twice()
            && ch.is_ascii_digit()
        {
            self.advance();
            self.advance();

            while let Some(ch) = self.peek()
                && ch.is_ascii_digit()
            {
                self.advance();
            }
        }

        let num = match self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>()
        {
            Ok(n) => n,
            Err(_) => {
                return Err(ScanError::UnexpectedCharacter {
                    line: self.line,
                    ch: '.',
                }
                .into());
            }
        };

        self.add_token_with_literal(TokenType::Number, Literal::Number(num));
        Ok(())
    }

    fn scan_identifier(&mut self) -> Result<(), RloxErr> {
        while let Some(ch) = self.peek()
            && (ch.is_alphanumeric() || ch == '_')
        {
            self.advance();
        }

        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        match match_keyword(&text) {
            Some(kw) => self.add_token(kw),
            None => self.add_token(TokenType::Identifier),
        }
        Ok(())
    }

    // ====================================================================================
    // scanner helpers
    // ====================================================================================

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let ch = self.source[self.current];
        self.current += 1;
        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_twice(&self) -> Option<char> {
        if (self.current + 1) >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn add_token(&mut self, tok: TokenType) {
        self.tokens.push(Token {
            ty: tok,
            lexeme: self.source[self.start..self.current].iter().collect(),
            literal: None,
            line: self.line,
        })
    }

    fn add_token_with_literal(&mut self, tok: TokenType, literal: Literal) {
        self.tokens.push(Token {
            ty: tok,
            lexeme: self.source[self.start..self.current].iter().collect(),
            literal: Some(literal),
            line: self.line,
        })
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

// ====================================================================================
// general helpers
// ====================================================================================

fn match_token(char: char) -> Option<TokenType> {
    use TokenType::*;

    match char {
        '(' => Some(LeftParen),
        ')' => Some(RightParen),
        '*' => Some(LeftBrace),
        '+' => Some(RightBrace),
        ',' => Some(Comma),
        '-' => Some(Dot),
        '.' => Some(Minus),
        ';' => Some(Plus),
        '{' => Some(Semicolon),
        '}' => Some(Star),
        _ => None,
    }
}

fn match_keyword(s: &str) -> Option<TokenType> {
    use TokenType::*;

    match s {
        "var" => Some(Var),
        "print" => Some(Print),

        "if" => Some(If),
        "else" => Some(Else),

        "while" => Some(While),
        "for" => Some(For),

        "fun" => Some(Fun),
        "return" => Some(Return),

        "true" => Some(True),
        "false" => Some(False),

        "and" => Some(And),
        "or" => Some(Or),

        "class" => Some(Class),
        "super" => Some(Super),
        "this" => Some(This),

        "nil" => Some(Nil),

        _ => None,
    }
}
