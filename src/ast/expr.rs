#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
}

impl Expr {
    pub fn bool(b: bool) -> Self {
        Expr::Literal(Literal::Bool(b))
    }
}
