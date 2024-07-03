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
