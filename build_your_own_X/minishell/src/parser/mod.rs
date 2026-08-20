pub mod ast;
pub mod error;
pub mod lexer;
#[allow(clippy::module_inception)]
pub mod parser;

pub use ast::*;
pub use error::*;

use lexer::tokenize;
use parser::Parser;

pub fn parse(input: &str) -> Result<AstNode, ParseError> {
    let tokens = tokenize(input)?;

    if tokens.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut parser = Parser::new(tokens);
    parser.parse_pipeline()
}
