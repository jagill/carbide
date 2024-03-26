use super::error::ParseError;
use super::tokenizer::Tokenizer;
use super::ParseResult;
use super::TokenType as ToT;
use crate::ast::expr::BinaryOp;
use crate::ast::expr::Expr;
use crate::ast::expr::UnaryOp;

pub struct Parser<'source> {
    tokenizer: Tokenizer<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(tokenizer: Tokenizer<'source>) -> Self {
        Self { tokenizer }
    }

    // Specific parsing for AST
    pub fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.log_or()
    }

    fn log_or(&mut self) -> ParseResult<Expr> {
        let left = self.log_and()?;

        if self.tokenizer.opt(ToT::Or).is_some() {
            return Ok(Expr::binary(left, BinaryOp::Or, self.log_or()?));
        }

        Ok(left)
    }

    fn log_and(&mut self) -> ParseResult<Expr> {
        let left = self.equality()?;

        if self.tokenizer.opt(ToT::And).is_some() {
            return Ok(Expr::binary(left, BinaryOp::And, self.log_and()?));
        }

        Ok(left)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let left = self.unary()?;

        if self.tokenizer.opt(ToT::BangEqual).is_some() {
            return Ok(Expr::binary(left, BinaryOp::NotEqual, self.unary()?));
        }

        if self.tokenizer.opt(ToT::EqualEqual).is_some() {
            return Ok(Expr::binary(left, BinaryOp::Equal, self.unary()?));
        }

        Ok(left)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.tokenizer.opt(ToT::Not).is_some() {
            return Ok(Expr::unary(UnaryOp::Not, self.unary()?));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        let token = self.tokenizer.force_advance()?;
        match token.token_type {
            ToT::False => Ok(Expr::bool(false)),
            ToT::True => Ok(Expr::bool(true)),
            _ => Err(ParseError::unexpected_token(token, "primary expression")),
        }
    }
}
