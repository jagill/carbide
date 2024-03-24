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
    pub token_type: TokenType,
    pub data: TokenData<'source>,
}

pub struct Tokenizer<'source> {
    lexer: Lexer<'source, TokenType>,
    /// Contain next Token, if any.  To interpret this,
    /// None -> We have not pulled the next token.
    /// Some(None) -> We have pulled the next token, but there isn't any.
    /// Some(Some(t)) -> We've pulled the next token, and this is it.
    dock: Option<Option<Token<'source>>>,
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
    /// An Err from lexer is converted to an UnknownToken.
    fn pull(&mut self) -> Option<Token<'source>> {
        let token_type_opt = self
            .lexer
            .next()
            .map(|res| res.unwrap_or(TokenType::UnknownToken));
        let data = TokenData {
            span: self.lexer.span(),
            lexeme: self.lexer.slice(),
            line: self.line,
            col: self.col,
        };
        match &token_type_opt {
            None => (),
            Some(TokenType::Newline) => {
                self.line += 1;
                self.col = 0;
            }
            Some(_) => self.col += self.lexer.span().end - self.lexer.span().start,
        }
        token_type_opt.map(|token_type| Token { token_type, data })
    }

    /// Produce the next token if any, or None if no tokens left.
    pub fn next(&mut self) -> Option<Token<'source>> {
        self.dock.take().unwrap_or_else(|| self.pull())
    }

    /// Produce the next token, skipping whitespace/newline.
    /// Return None if no tokens are left.
    pub fn advance(&mut self) -> Option<Token<'source>> {
        loop {
            let token = self.next()?;
            match token.token_type {
                TokenType::Newline | TokenType::Whitespace => continue,
                _ => return Some(token),
            }
        }
    }

    /// Advance and return Token.
    /// Return ParseError::UnexpectedEnd if no tokens are left.
    pub fn force_advance(&mut self) -> ParseResult<Token<'source>> {
        match self.advance() {
            None => Err(ParseError::Eof {
                line: self.line,
                col: self.col,
            }),
            Some(
                token @ Token {
                    token_type: TokenType::UnknownToken,
                    ..
                },
            ) => Err(ParseError::unknown_token(token)),
            Some(token) => Ok(token),
        }
    }

    /// Produce the next token if it is of the given type.
    /// Return ParseError::UnexpectedToken if the next token is not of the given type.
    /// Return ParseError::UnexpectedEnd if no tokens are left.
    pub fn expect(&mut self, expected: TokenType) -> ParseResult<Token> {
        match self.force_advance()? {
            token if token.token_type == expected => Ok(token),
            token => Err(ParseError::unexpected_token(token, format!("{expected:?}"))),
        }
    }

    /// Produce the next token if it is of the given type.
    /// Return None if the next token is not of the given type; this does not consume the token.
    /// Return None if no tokens are left.
    pub fn opt(&mut self, expected: TokenType) -> Option<Token> {
        let token = self.advance()?;
        assert!(self.dock.is_none(), "Expected empty dock after advance",);

        if token.token_type == expected {
            Some(token)
        } else {
            self.dock = Some(Some(token));
            None
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
    use TokenType::*;

    #[test]
    fn test_peek() {
        let mut tokenizer = Tokenizer::new(TokenType::lexer("bool true"));
        assert_eq!(tokenizer.opt(False), None);
        let t = tokenizer.opt(Bool).unwrap();
        assert_eq!(t.token_type, Bool);
        assert_eq!(t.data.lexeme, "bool");
        assert_eq!(t.data.span, 0..4);
        assert_eq!(t.data.line, 0);
        assert_eq!(t.data.col, 0);
        let t = tokenizer.next().unwrap();
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
        match tokenizer.force_advance() {
            Err(e) => assert_eq!(e, ParseError::Eof { line: 0, col: 9 }),
            x => panic!("Unexpected result: {x:?}"),
        }
    }

    #[test]
    fn test_double_unary() {
        let mut tokenizer = Tokenizer::new(TokenType::lexer("not not false"));
        let mut t: Token;
        t = tokenizer.advance().unwrap();
        assert_eq!(t.token_type, Not);
        t = tokenizer.advance().unwrap();
        assert_eq!(t.token_type, Not);
        t = tokenizer.advance().unwrap();
        assert_eq!(t.token_type, False);
    }

    #[test]
    fn test_double_unary_opt() {
        let mut tokenizer = Tokenizer::new(TokenType::lexer("not not false"));
        match tokenizer.opt(Not) {
            None => panic!("Expected first Not"),
            Some(t) => assert_eq!(t.token_type, Not),
        }
        match tokenizer.opt(Not) {
            None => panic!("Expected second Not"),
            Some(t) => assert_eq!(t.token_type, Not),
        }
        match tokenizer.opt(Not) {
            None => (),
            Some(t) => panic!("Unexpected token {t:?}"),
        }
        let t = tokenizer.advance().unwrap();
        assert_eq!(t.token_type, False);
    }
}
