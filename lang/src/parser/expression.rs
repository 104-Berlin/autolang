use std::fmt::Display;

use crate::{spanned::Spanned, tokenizer::literal::Literal};

use super::{binary_expression::BinaryExpression, type_def::TypeID};

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(Spanned<String>, Spanned<Vec<Spanned<Expr>>>),
    Binary(Spanned<BinaryExpression>),

    Literal(Spanned<Literal>),
    Variable(Spanned<String>),

    Assignment(Spanned<String>, Box<Spanned<Expr>>),
    Let(Spanned<String>, Spanned<TypeID>, Box<Spanned<Expr>>),

    Block(Vec<Spanned<Expr>>, Option<Box<Spanned<Expr>>>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FunctionCall(name, _) => write!(f, "{}()", name.value),
            Expr::Binary(expr) => {
                write!(
                    f,
                    "({} {} {})",
                    expr.value.lhs.value, expr.value.op.value, expr.value.rhs.value
                )
            }
            Expr::Assignment(var, expr) => write!(f, "{} = {}", var.value, expr.value),
            Expr::Let(var, type_id, assign) => {
                write!(f, "let {}: {} = {}", var.value, type_id.value, assign.value)
            }
            Expr::Literal(literal) => write!(f, "{}", literal.value),
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
