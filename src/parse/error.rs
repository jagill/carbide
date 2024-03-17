use logos::Lexer;
use thiserror::Error;
use super::Token;
use super::LexError;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Expected {expected}, but ran out of tokens.")]
    UnexpectedEnd { expected: String },
    #[error("Expected {expected} on line {line}, but found {actual:?} '{lexeme}'.")]
    UnexpectedToken {
        actual: TokenType,
        line: usize,
        lexeme: String,
        expected: String,
    },
}

impl ParseError {
    pub fn end(msg: impl Into<String>) -> Self {
        ParseError::UnexpectedEnd {
            expected: msg.into(),
        }
    }

    // pub fn wrong_token(token: Token, : Lexer, msg: impl Into<String>) -> Self {
    //     ParseError::UnexpectedToken {
    //         actual: .typ,
    //         line: token.line,
    //         lexeme: token.lexeme.to_owned(),
    //         expected: msg.into(),
    //     }
    // }
}
