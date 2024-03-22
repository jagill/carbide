use super::error::ParseError;
use super::tokenizer::Tokenizer;
use super::ParseResult;
use super::Token;
use super::TokenType;
use crate::ast::expr::Expr;

pub struct Parser<'source> {
    tokenizer: Tokenizer<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(tokenizer: Tokenizer<'source>) -> Self {
        Self { tokenizer }
    }

    // Specific parsing for AST
    pub fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        use TokenType::*;

        let token = self.advance()?;
        match token.token_type {
            False => Ok(Expr::bool(false)),
            True => Ok(Expr::bool(true)),
            _ => Err(ParseError::unexpected_token(token, "primary expression")),
        }
    }

    // Generic parsing methods
    fn advance(&mut self) -> ParseResult<Token> {
        loop {
            let token = self.tokenizer.advance()?;
            match token.token_type {
                TokenType::Newline | TokenType::Whitespace => continue,
                _ => return Ok(token),
            }
        }
    }
}
