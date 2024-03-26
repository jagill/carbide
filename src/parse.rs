mod error;
pub mod parser;
mod token_type;
mod tokenizer;

pub use token_type::TokenType;
pub use tokenizer::Token;
pub use tokenizer::TokenData;
pub use tokenizer::Tokenizer;

use crate::ast::expr::Expr;
use error::ParseError;
use logos::Logos;

type ParseResult<T> = Result<T, ParseError>;

pub fn parse_expr(source: &str) -> ParseResult<Expr> {
    let lex = TokenType::lexer(source);
    let tokenizer = Tokenizer::new(lex);
    let mut parser = parser::Parser::new(tokenizer);
    parser.parse_expression()
}

#[cfg(test)]
mod tests;
