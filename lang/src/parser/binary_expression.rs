use super::Expr;

pub enum BinaryOperator {}

pub struct BinaryExpression {
    left: Box<Expr>,
    operator: BinaryOperator,
    right: Box<Expr>,
}
