use crate::parser::WordPart;
use crate::parser::ast::{AstNode, Command, Redirect, Word};
use crate::parser::error::ParseError;
use crate::parser::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn is_word_token(tok: &Token) -> bool {
        matches!(
            tok,
            Token::Literal(_) | Token::VarExpand(_) | Token::LastStatus
        )
    }

    fn parse_word(&mut self) -> Result<Word, ParseError> {
        let mut parts = Vec::new();

        while let Some(tok) = self.peek() {
            match tok {
                Token::Literal(s) => parts.push(WordPart::Literal(s.clone())),
                Token::VarExpand(v) => parts.push(WordPart::Var(v.clone())),
                Token::LastStatus => parts.push(WordPart::LastStatus),
                _ => break,
            }

            self.next();
        }

        if parts.is_empty() {
            Err(ParseError::ExpectedWord)
        } else {
            Ok(Word { parts })
        }
    }

    fn parse_redirect(&mut self) -> Result<Redirect, ParseError> {
        match self.peek() {
            Some(Token::RedirectOut) => {
                self.next(); // consume '>'
                Ok(Redirect::Output(self.parse_word()?))
            }

            Some(Token::RedirectAppend) => {
                self.next(); // consume '>'
                Ok(Redirect::Append(self.parse_word()?))
            }

            Some(Token::RedirectIn) => {
                self.next(); // consume '>'
                Ok(Redirect::Input(self.parse_word()?))
            }

            Some(Token::RedirectErr) => {
                self.next();
                Ok(Redirect::ErrorOutput(self.parse_word()?))
            }

            Some(Token::RedirectErrAppend) => {
                self.next();
                Ok(Redirect::ErrorAppend(self.parse_word()?))
            }

            _ => Ok(Redirect::None),
        }
    }

    fn parse_command(&mut self) -> Result<AstNode, ParseError> {
        let name = self.parse_word()?;

        let mut args = Vec::new();
        let mut redirect = Redirect::None;

        loop {
            if let Some(Token::WordSep) = self.peek() {
                self.next();
            }

            match self.peek() {
                Some(tok) if Self::is_word_token(tok) => {
                    args.push(self.parse_word()?);
                }

                Some(Token::RedirectOut)
                | Some(Token::RedirectAppend)
                | Some(Token::RedirectIn)
                | Some(Token::RedirectErr)
                | Some(Token::RedirectErrAppend) => {
                    redirect = self.parse_redirect()?;
                }

                Some(Token::Pipe) | None => break,

                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        Ok(AstNode::Command(Command {
            name,
            args,
            redirect,
        }))
    }

    pub fn parse_pipeline(&mut self) -> Result<AstNode, ParseError> {
        let left = self.parse_command()?;

        if let Some(Token::Pipe) = self.peek() {
            self.next(); // consume pipe
            if self.peek().is_none() {
                return Err(ParseError::PipeMissingRhs);
            }
            let right = self.parse_pipeline()?;

            return Ok(AstNode::Pipeline(Box::new(left), Box::new(right)));
        }

        Ok(left)
    }
}
