use std::fmt::Display;

use crate::tokenizer::Literal;

use super::binary_expression::BinaryExpression;

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(String),
    Binary(BinaryExpression),

    Literal(Literal),
    Variable(String),
}

impl Expr {
    pub fn evalutae(&self) -> i64 {
        match self {
            Expr::FunctionCall(name) | Expr::Variable(name) => {
                println!(
                    "Function call / Variable not implemented. Evaluating '{}' to 0",
                    name
                );
                0
            }
            Expr::Literal(literal) => match literal {
                Literal::NumberInt(val) => *val as i64,
                Literal::NumberFloat(val) => {
                    println!("Evaluating float to int: {}", val);
                    val.trunc() as i64
                }
            },
            Expr::Binary(BinaryExpression { lhs, op, rhs }) => {
                let lhs = lhs.evalutae();
                let rhs = rhs.evalutae();

                match op {
                    crate::parser::binary_expression::BinaryOperator::Add => lhs + rhs,
                    crate::parser::binary_expression::BinaryOperator::Substract => lhs - rhs,
                    crate::parser::binary_expression::BinaryOperator::Multiply => lhs * rhs,
                    crate::parser::binary_expression::BinaryOperator::Divide => lhs / rhs,
                }
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FunctionCall(name) => write!(f, "{}()", name),
            Expr::Binary(expr) => {
                write!(f, "({} {} {})", expr.lhs, expr.op, expr.rhs)
            }
            Expr::Literal(literal) => write!(
                f,
                "{}",
                match literal {
                    Literal::NumberFloat(val) => val.to_string(),
                    Literal::NumberInt(val) => val.to_string(),
                }
            ),
            Expr::Variable(name) => write!(f, "{}", name),
        }
    }
}
