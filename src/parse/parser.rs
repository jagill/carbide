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
        if self.match_next(ToT::If) {
            self.parse_if()
        } else {
            self.log_or()
        }
    }

    // If Token must already be consumed
    fn parse_if(&mut self) -> ParseResult<Expr> {
        let condition = self.parse_expression()?;
        self.tokenizer.expect(ToT::OpenParen)?;
        let then_expr = self.block()?;
        let else_expr = if self.match_next(ToT::Else) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Expr::If {
            condition: Box::new(condition),
            then_expr: Box::new(then_expr),
            else_expr: else_expr.map(Box::new),
        })
    }

    fn log_or(&mut self) -> ParseResult<Expr> {
        let left = self.log_and()?;

        if self.match_next(ToT::Or) {
            return Ok(Expr::binary(left, BinaryOp::Or, self.log_or()?));
        }

        Ok(left)
    }

    fn log_and(&mut self) -> ParseResult<Expr> {
        let left = self.equality()?;

        if self.match_next(ToT::And) {
            return Ok(Expr::binary(left, BinaryOp::And, self.log_and()?));
        }

        Ok(left)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let left = self.term()?;

        if self.match_next(ToT::BangEqual) {
            return Ok(Expr::binary(left, BinaryOp::NotEqual, self.term()?));
        }

        if self.match_next(ToT::EqualEqual) {
            return Ok(Expr::binary(left, BinaryOp::Equal, self.term()?));
        }

        Ok(left)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let left = self.factor()?;

        if self.match_next(ToT::Minus) {
            return Ok(Expr::binary(left, BinaryOp::Sub, self.term()?));
        }

        if self.match_next(ToT::Plus) {
            return Ok(Expr::binary(left, BinaryOp::Add, self.term()?));
        }

        Ok(left)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let left = self.unary()?;

        if self.match_next(ToT::Slash) {
            return Ok(Expr::binary(left, BinaryOp::Div, self.factor()?));
        }

        if self.match_next(ToT::Star) {
            return Ok(Expr::binary(left, BinaryOp::Mult, self.factor()?));
        }

        Ok(left)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.match_next(ToT::Not) {
            return Ok(Expr::unary(UnaryOp::Not, self.unary()?));
        }
        if self.match_next(ToT::Minus) {
            return Ok(Expr::unary(UnaryOp::Neg, self.unary()?));
        }
        if self.match_next(ToT::Plus) {
            return Ok(Expr::unary(UnaryOp::Pos, self.unary()?));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        let token = self.tokenizer.force_advance()?;
        match token.token_type {
            ToT::False => Ok(Expr::bool(false)),
            ToT::True => Ok(Expr::bool(true)),
            ToT::Int => {
                let i: i64 = token.data.lexeme.parse().unwrap();
                Ok(Expr::int(i))
            }
            ToT::OpenParen => self.block(),
            _ => Err(ParseError::unexpected_token(token, "primary expression")),
        }
    }

    // This expects the open delimiter to already be consumed
    // TODO: handle semicolons
    fn block(&mut self) -> ParseResult<Expr> {
        let mut contents = Vec::new();
        contents.push(self.parse_expression()?);
        self.tokenizer.expect(ToT::CloseParen)?;
        Ok(Expr::Block(contents))
    }

    // Generic Parsing functions
    fn match_next(&mut self, token_type: ToT) -> bool {
        self.tokenizer.opt(token_type).is_some()
    }
}
