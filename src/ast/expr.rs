#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary { op: UnaryOp, right: Box<Expr> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
}

impl Expr {
    pub fn bool(b: bool) -> Self {
        Expr::Literal(Literal::Bool(b))
    }

    pub fn unary(op: UnaryOp, right: Expr) -> Self {
        Expr::Unary {
            op,
            right: Box::new(right),
        }
    }
}
