use super::parse_expr;
use super::Expr;
use super::ParseError;
use super::TokenType;
use crate::ast::expr::BinaryOp;
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

#[test]
fn test_parse_and_or() {
    assert_eq!(
        parse_expr(" true and true"),
        Ok(Expr::binary(
            Expr::bool(true),
            BinaryOp::And,
            Expr::bool(true)
        ))
    );
    assert_eq!(
        parse_expr("not true and false"),
        Ok(Expr::binary(
            Expr::unary(UnaryOp::Not, Expr::bool(true)),
            BinaryOp::And,
            Expr::bool(false)
        ))
    );
    assert_eq!(
        parse_expr("not not true and not not false"),
        Ok(Expr::binary(
            Expr::unary(UnaryOp::Not, Expr::unary(UnaryOp::Not, Expr::bool(true))),
            BinaryOp::And,
            Expr::unary(UnaryOp::Not, Expr::unary(UnaryOp::Not, Expr::bool(false))),
        ))
    );

    assert_eq!(
        parse_expr("true or false"),
        Ok(Expr::binary(
            Expr::bool(true),
            BinaryOp::Or,
            Expr::bool(false),
        ))
    );
    assert_eq!(
        parse_expr("true or false and not true or false"),
        Ok(Expr::binary(
            Expr::bool(true),
            BinaryOp::Or,
            Expr::binary(
                Expr::binary(
                    Expr::bool(false),
                    BinaryOp::And,
                    Expr::unary(UnaryOp::Not, Expr::bool(true))
                ),
                BinaryOp::Or,
                Expr::bool(false),
            )
        ))
    );
}

#[test]
fn test_equals() {
    assert_eq!(
        parse_expr("true == false"),
        Ok(Expr::binary(
            Expr::bool(true),
            BinaryOp::Equal,
            Expr::bool(false),
        ))
    );
    assert_eq!(
        parse_expr("true != false"),
        Ok(Expr::binary(
            Expr::bool(true),
            BinaryOp::NotEqual,
            Expr::bool(false),
        ))
    );

    assert_eq!(
        parse_expr("true != false or true == true"),
        Ok(Expr::binary(
            Expr::binary(Expr::bool(true), BinaryOp::NotEqual, Expr::bool(false),),
            BinaryOp::Or,
            Expr::binary(Expr::bool(true), BinaryOp::Equal, Expr::bool(true),)
        ))
    );
}

#[test]
fn test_parse_block() {
    assert_eq!(
        parse_expr("()"),
        Err(ParseError::UnexpectedToken {
            expected: "primary expression".to_owned(),
            actual: TokenType::CloseParen,
            lexeme: ")".to_owned(),
            line: 0,
            col: 1,
        })
    );

    assert_eq!(
        parse_expr("(true)"),
        Ok(Expr::Block(vec![Expr::bool(true)]))
    );

    assert_eq!(
        parse_expr("(true or false)"),
        Ok(Expr::Block(vec![Expr::binary(
            Expr::bool(true),
            BinaryOp::Or,
            Expr::bool(false),
        ),]))
    );

    assert_eq!(
        parse_expr("false and (true or false)"),
        Ok(Expr::binary(
            Expr::bool(false),
            BinaryOp::And,
            Expr::Block(vec![Expr::binary(
                Expr::bool(true),
                BinaryOp::Or,
                Expr::bool(false)
            ),])
        ))
    );
}

#[test]
fn test_if_exprs() {
    assert_eq!(
        parse_expr("if true (false)"),
        Ok(Expr::ifthen(
            Expr::bool(true),
            Expr::Block(vec![Expr::bool(false)]),
            None,
        ))
    );

    assert_eq!(
        parse_expr("if true (false) else true or false"),
        Ok(Expr::ifthen(
            Expr::bool(true),
            Expr::Block(vec![Expr::bool(false)]),
            Some(Expr::binary(
                Expr::bool(true),
                BinaryOp::Or,
                Expr::bool(false),
            )),
        ))
    );

    assert_eq!(
        parse_expr("if true (false) else if true (true) else false"),
        Ok(Expr::ifthen(
            Expr::bool(true),
            Expr::Block(vec![Expr::bool(false)]),
            Some(Expr::ifthen(
                Expr::bool(true),
                Expr::Block(vec![Expr::bool(true)]),
                Some(Expr::bool(false)),
            ))
        ))
    );
}
