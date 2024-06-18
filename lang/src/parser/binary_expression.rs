use std::fmt::Display;

use super::{expression::SpannedExpr, spans::Spanned, Expr};

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression<'a> {
    pub lhs: Box<SpannedExpr<'a>>,
    pub op: Spanned<'a, BinaryOperator>,
    pub rhs: Box<SpannedExpr<'a>>,
}

impl<'a> BinaryExpression<'a> {
    pub fn new(
        lhs: SpannedExpr<'a>,
        op: Spanned<'a, BinaryOperator>,
        rhs: SpannedExpr<'a>,
    ) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl BinaryOperator {
    pub fn precedence(&self) -> i16 {
        match self {
            BinaryOperator::Add | BinaryOperator::Substract => 10,
            BinaryOperator::Multiply | BinaryOperator::Divide => 20,
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Substract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
        }
    }
}
