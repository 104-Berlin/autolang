use std::fmt::Display;

use crate::tokenizer::literal::Literal;

use super::binary_expression::BinaryExpression;

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(String),
    Binary(BinaryExpression),

    Literal(Literal),
    Variable(String),

    Assignment(String, Box<Expr>),

    Block(Vec<Expr>, Option<Box<Expr>>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::FunctionCall(name) | Expr::Variable(name) => {
                println!(
                    "Function call / Variable not implemented. Evaluating '{}' to 0",
                    name
                );
                0
            }
            Expr::Literal(literal) => match literal {
                Literal::NumberInt(val) => *val,
                Literal::NumberFloat(val) => {
                    println!("Evaluating float to int: {}", val);
                    val.trunc() as i64
                }
            },
            Expr::Assignment(var, expr) => {
                let val = expr.evaluate();
                println!("Assigning {} to {}", val, var);
                eprintln!("Evaluating and assignment is not implemented yet!");
                0
            }
            Expr::Binary(BinaryExpression { lhs, op, rhs }) => {
                let lhs = lhs.evaluate();
                let rhs = rhs.evaluate();

                match op {
                    crate::parser::binary_expression::BinaryOperator::Add => lhs + rhs,
                    crate::parser::binary_expression::BinaryOperator::Substract => lhs - rhs,
                    crate::parser::binary_expression::BinaryOperator::Multiply => lhs * rhs,
                    crate::parser::binary_expression::BinaryOperator::Divide => lhs / rhs,
                }
            }
            Expr::Block(expr, return_expr) => {
                for e in expr {
                    e.evaluate();
                }
                if let Some(return_expr) = return_expr {
                    return_expr.evaluate()
                } else {
                    0
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
            Expr::Assignment(var, expr) => write!(f, "{} = {}", var, expr),
            Expr::Literal(literal) => write!(
                f,
                "{}",
                match literal {
                    Literal::NumberFloat(val) => val.to_string(),
                    Literal::NumberInt(val) => val.to_string(),
                }
            ),
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::Block(expr, return_expr) => {
                write!(f, "{{")?;
                for e in expr {
                    write!(f, "{}, ", e)?;
                }
                if let Some(return_expr) = return_expr {
                    write!(f, "{}", return_expr)?;
                }
                write!(f, "}}")
            }
        }
    }
}
