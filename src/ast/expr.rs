#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    And,
    Or,
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

    pub fn binary(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}
