use logos::Lexer;
use std::ops::Range;

use super::token_type::TokenType;
use super::ParseResult;
use crate::parse::error::ParseError;

/// Token data (lexeme etc)
#[derive(Debug, Clone, PartialEq)]
pub struct TokenData<'source> {
    /// char index span in source string
    pub span: Range<usize>,
    pub lexeme: &'source str,
    /// line (0-index) the token is on
    pub line: usize,
    /// column (0-index) the token starts
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'source> {
    token_type: TokenType,
    data: TokenData<'source>,
}

pub struct Tokenizer<'source> {
    lexer: Lexer<'source, TokenType>,
    dock: Option<Token<'source>>,
    /// line (0-index) the next token is on
    line: usize,
    /// column (0-index) the next token starts
    col: usize,
}

// Tokenizer allows common operations for tokens
impl<'source> Tokenizer<'source> {
    pub fn new(lexer: Lexer<'source, TokenType>) -> Self {
        Self {
            lexer,
            dock: None,
            line: 0,
            col: 0,
        }
    }

    /// Pull the next TokenData from lexer; does not check dock
    fn pull(&mut self) -> ParseResult<Token<'source>> {
        let token_type = match self.lexer.next() {
            None => Err(ParseError::Eof {
                line: self.line,
                col: self.col,
            }),
            // Logos returns an Err(()) for unknown token
            Some(Err(())) => Err(ParseError::UnknownToken {
                lexeme: self.lexer.slice().to_owned(),
                line: self.line,
                col: self.col,
            }),
            Some(Ok(t)) => Ok(t),
        }?;
        let data = TokenData {
            span: self.lexer.span(),
            lexeme: self.lexer.slice(),
            line: self.line,
            col: self.col,
        };
        match &token_type {
            TokenType::Newline => {
                self.line += 1;
                self.col = 0;
            }
            _ => self.col += self.lexer.span().end - self.lexer.span().start,
        }
        Ok(Token { token_type, data })
    }

    fn load(&mut self) -> ParseResult<()> {
        let empty = self.dock.is_none();
        if empty {
            let next_token = self.pull()?;
            self.dock = Some(next_token);
        }
        Ok(())
    }

    /// Produce the next token, or an error.
    /// Return ParseError::UnexpectedEnd if no tokens are left.
    pub fn advance(&mut self) -> ParseResult<Token<'source>> {
        if let Some(token_res) = self.dock.take() {
            Ok(token_res)
        } else {
            self.pull()
        }
    }

    /// Produce the next token if it is of the given type.
    /// Return ParseError::UnexpectedToken if the next token is not of the given type.
    /// Return ParseError::UnexpectedEnd if no tokens are left.
    pub fn expect(&mut self, expected: TokenType) -> ParseResult<Token> {
        match self.advance()? {
            token if token.token_type == expected => Ok(token),
            Token {
                token_type: tp,
                data,
            } => Err(ParseError::UnexpectedToken {
                actual: tp,
                line: data.line,
                col: data.col,
                lexeme: data.lexeme.to_owned(),
                expected: format!("{expected:?}"),
            }),
        }
    }

    /// Produce the next token if it is of the given type.
    /// Return None if the next token is not of the given type; this does not consume the token.
    /// Return None if no tokens are left.
    pub fn opt(&mut self, expected: TokenType) -> ParseResult<Option<Token>> {
        self.load()?;

        match self.dock.take() {
            Some(token) if token.token_type == expected => Ok(Some(token)),
            token_opt => {
                self.dock = token_opt;
                Ok(None)
            }
        }
    }

    // /// Return a string describing the current token.
    // pub fn report(&self) -> String {
    //     format!("'{}' [{:?}]", self.slice(), self.span())
    // }

    // /// Return the &str of the current token.
    // pub fn slice(&self) -> &'source str {
    //     self._slice
    // }

    // /// Return the span in the source str of the current token.
    // pub fn span(&self) -> Range<usize> {
    //     self._span.clone()
    // }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_peek() {
        use TokenType::*;

        let mut tokenizer = Tokenizer::new(TokenType::lexer("bool true"));
        assert_eq!(tokenizer.opt(False), Ok(None));
        let t = tokenizer.opt(Bool).unwrap().unwrap();
        assert_eq!(t.token_type, Bool);
        assert_eq!(t.data.lexeme, "bool");
        assert_eq!(t.data.span, 0..4);
        assert_eq!(t.data.line, 0);
        assert_eq!(t.data.col, 0);
        let t = tokenizer.advance().unwrap();
        assert_eq!(t.token_type, Whitespace);
        assert_eq!(t.data.lexeme, " ");
        assert_eq!(t.data.span, 4..5);
        assert_eq!(t.data.line, 0);
        assert_eq!(t.data.col, 4);
        let t = tokenizer.advance().unwrap();
        assert_eq!(t.token_type, True);
        assert_eq!(t.data.lexeme, "true");
        assert_eq!(t.data.span, 5..9);
        assert_eq!(t.data.line, 0);
        assert_eq!(t.data.col, 5);
        match tokenizer.advance() {
            Err(e) => assert_eq!(e, ParseError::Eof { line: 0, col: 9 }),
            x => panic!("Unexpected result: {x:?}"),
        }
    }
}
