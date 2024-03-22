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
mod tests {
    use super::*;
    use crate::ast::expr::UnaryOp;

    #[test]
    fn test_parse_errors() {
        assert_eq!(
            parse_expr("  foo"),
            Err(ParseError::UnexpectedToken {
                expected: "primary expression".to_owned(),
                actual: TokenType::Identifier,
                lexeme: "foo".to_owned(),
                line: 0,
                col: 2
            })
        );
        assert_eq!(
            parse_expr("\n !"),
            Err(ParseError::UnknownToken {
                lexeme: "!".to_owned(),
                line: 1,
                col: 1,
            })
        );
    }

    #[test]
    fn test_parse_primary() {
        assert_eq!(parse_expr("true"), Ok(Expr::bool(true)));
        assert_eq!(parse_expr("false"), Ok(Expr::bool(false)));
    }

    #[test]
    fn test_parse_unary() {
        assert_eq!(
            parse_expr("not true"),
            Ok(Expr::unary(UnaryOp::Not, Expr::bool(true)))
        );
        assert_eq!(
            parse_expr("not not false"),
            Ok(Expr::unary(
                UnaryOp::Not,
                Expr::unary(UnaryOp::Not, Expr::bool(false))
            ))
        );
    }
}
