use std::fmt::Display;

use crate::{error::Spanned, tokenizer::literal::Literal};

use super::{binary_expression::BinaryExpression, type_def::TypeID};

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(Spanned<String>),
    Binary(Spanned<BinaryExpression>),

    Literal(Spanned<Literal>),
    Variable(Spanned<String>),

    Assignment(Spanned<String>, Box<Spanned<Expr>>),
    Let(Spanned<String>, Spanned<TypeID>),

    Block(Vec<Spanned<Expr>>, Option<Box<Spanned<Expr>>>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::FunctionCall(name) | Expr::Variable(name) => {
                println!(
                    "Function call / Variable not implemented. Evaluating '{}' to 0",
                    name.value
                );
                0
            }
            Expr::Literal(literal) => match literal.value {
                Literal::NumberInt(val) => val,
                Literal::NumberFloat(val) => {
                    println!("Evaluating float to int: {}", val);
                    val.trunc() as i64
                }
            },
            Expr::Assignment(var, expr) => {
                let val = expr.value.evaluate();
                println!("Assigning {} to {}", val, var.value);
                eprintln!("Evaluating and assignment is not implemented yet!");
                0
            }
            Expr::Let(_, _) => 0,
            Expr::Binary(Spanned::<BinaryExpression> {
                value: BinaryExpression { lhs, op, rhs },
                ..
            }) => {
                let lhs = lhs.value.evaluate();
                let rhs = rhs.value.evaluate();

                match op.value {
                    crate::parser::binary_expression::BinaryOperator::Add => lhs + rhs,
                    crate::parser::binary_expression::BinaryOperator::Substract => lhs - rhs,
                    crate::parser::binary_expression::BinaryOperator::Multiply => lhs * rhs,
                    crate::parser::binary_expression::BinaryOperator::Divide => lhs / rhs,
                }
            }
            Expr::Block(expr, return_expr) => {
                for e in expr {
                    e.value.evaluate();
                }
                if let Some(return_expr) = return_expr {
                    return_expr.value.evaluate()
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
            Expr::FunctionCall(name) => write!(f, "{}()", name.value),
            Expr::Binary(expr) => {
                write!(
                    f,
                    "({} {} {})",
                    expr.value.lhs.value, expr.value.op.value, expr.value.rhs.value
                )
            }
            Expr::Assignment(var, expr) => write!(f, "{} = {}", var.value, expr.value),
            Expr::Let(var, type_id) => write!(f, "let {}: {}", var.value, type_id.value),
            Expr::Literal(literal) => write!(
                f,
                "{}",
                match literal.value {
                    Literal::NumberFloat(val) => val.to_string(),
                    Literal::NumberInt(val) => val.to_string(),
                }
            ),
            Expr::Variable(name) => write!(f, "{}", name.value),
            Expr::Block(expr, return_expr) => {
                write!(f, "{{")?;
                for e in expr {
                    write!(f, "{}, ", e.value)?;
                }
                if let Some(return_expr) = return_expr {
                    write!(f, "{}", return_expr.value)?;
                }
                write!(f, "}}")
            }
        }
    }
}
