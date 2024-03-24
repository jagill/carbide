use super::Token;
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

impl ParseError {
    pub fn unexpected_token(token: Token, msg: impl Into<String>) -> Self {
        ParseError::UnexpectedToken {
            expected: msg.into(),
            actual: token.token_type,
            lexeme: token.data.lexeme.to_owned(),
            line: token.data.line,
            col: token.data.col,
        }
    }

    pub fn unknown_token(token: Token) -> Self {
        assert_eq!(token.token_type, TokenType::UnknownToken);
        ParseError::UnknownToken {
            lexeme: token.data.lexeme.to_owned(),
            line: token.data.line,
            col: token.data.col,
        }
    }
}
