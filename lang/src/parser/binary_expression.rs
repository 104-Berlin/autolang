use std::fmt::Display;

use super::Expr;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}

impl BinaryExpression {
    pub fn new(left: Expr, operator: BinaryOperator, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
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
