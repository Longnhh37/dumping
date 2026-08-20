use crate::{
    ast::Expr,
    error::{RloxErr, parse_error::ParseError},
    lexer::{literal::Literal, token::Token, token_type::TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, RloxErr> {
        self.expression()
    }

    // ===============================================================================
    // precedence (low to high)
    // expression  →  equality
    // equality    →  comparison ( ("==" | "!=") comparison )*
    // comparison  →  term ( ("<" | "<=" | ">" | ">=") term )*
    // term        →  factor ( ("+" | "-") factor )*
    // factor      →  unary ( ("*" | "/") unary )*
    // unary       →  ("!" | "-") unary | primary
    // primary     →  NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    // ===============================================================================

    fn expression(&mut self) -> Result<Expr, RloxErr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, RloxErr> {
        let mut left = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, RloxErr> {
        let mut left = self.term()?;

        while self.match_tokens(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, RloxErr> {
        let mut left = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, RloxErr> {
        let mut left = self.unary()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, RloxErr> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, RloxErr> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.clone().unwrap()));
        }
        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expected ')' after expression")?;
            return Ok(Expr::Grouping {
                expressing: Box::new(expr),
            });
        }

        Err(ParseError::UnexpectedToken {
            token: self.peek().clone(),
        }
        .into())
    }

    // ===============================================================================
    // helpers
    // ===============================================================================

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for &ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ty: TokenType) -> bool {
        !self.is_at_end() && self.peek().ty == ty
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, ty: TokenType, msg: &str) -> Result<&Token, RloxErr> {
        if self.check(ty) {
            Ok(self.advance())
        } else {
            Err(ParseError::ExpectedToken {
                expected: ty,
                message: msg.into(),
            }
            .into())
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }
}
