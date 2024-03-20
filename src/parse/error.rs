use super::TokenType;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("unknown token '{lexeme}' at {line}:{col}")]
    UnknownToken {
        lexeme: String,
        line: usize,
        col: usize,
    },
    #[error("EOF reached at {line}:{col}")]
    Eof { line: usize, col: usize },
    #[error("expected {expected} at {line}:{col}, but found {actual:?} '{lexeme}'")]
    UnexpectedToken {
        expected: String,
        actual: TokenType,
        lexeme: String,
        line: usize,
        col: usize,
    },
    #[error("Unclassified error: {0}")]
    Unclassified(String),
}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError::Unclassified(s)
    }
}

// impl ParseError {
//     pub fn end(msg: impl Into<String>) -> Self {
//         ParseError::UnexpectedEnd {
//             expected: msg.into(),
//         }
//     }

// pub fn wrong_token(token: Token, : Lexer, msg: impl Into<String>) -> Self {
//     ParseError::UnexpectedToken {
//         actual: .typ,
//         line: token.line,
//         lexeme: token.lexeme.to_owned(),
//         expected: msg.into(),
//     }
// }
// }
