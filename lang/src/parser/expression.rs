use std::fmt::Display;

use crate::{spanned::Spanned, tokenizer::literal::Literal};

use super::{binary_expression::BinaryExpression, type_def::TypeID};

pub type IfCondition = (Box<Spanned<Expr>>, Box<Spanned<Expr>>);

#[derive(Debug, Clone)]
pub enum DotExpr {
    FunctionCall(Spanned<String>, Vec<Spanned<Expr>>),
    Variable(Spanned<String>),
}

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    /// A connected series of expressions combined with dot.
    /// # Example
    /// ```rs
    /// a.b().c.d(e, f)
    /// ```
    Dot {
        lhs: Box<Spanned<Expr>>,
        rhs: Spanned<DotExpr>,
    },

    FunctionCall(Spanned<String>, Vec<Spanned<Expr>>),
    Binary(Spanned<BinaryExpression>),

    Literal(Spanned<Literal>),
    StructLiteral(Spanned<String>, Vec<(Spanned<String>, Spanned<Expr>)>),
    Variable(Spanned<String>),

    Assignment(Spanned<String>, Box<Spanned<Expr>>),

    Let(Spanned<String>, Spanned<TypeID>, Box<Spanned<Expr>>),

    IfExpression {
        if_block: IfCondition,
        // Pair of condition and block
        else_if_blocks: Vec<IfCondition>,
        else_block: Option<Box<Spanned<Expr>>>,
    },

    Loop(Box<Spanned<Expr>>),

    Block(Vec<Spanned<Expr>>, Option<Box<Spanned<Expr>>>),

    Return(Option<Box<Spanned<Expr>>>),
    Break,
    Continue,
}

impl From<DotExpr> for Expr {
    fn from(value: DotExpr) -> Self {
        match value {
            DotExpr::FunctionCall(name, args) => Expr::FunctionCall(name, args),
            DotExpr::Variable(name) => Expr::Variable(name),
        }
    }
}

impl Display for DotExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotExpr::FunctionCall(name, args) => {
                write!(
                    f,
                    "{}({})",
                    name.value,
                    args.iter()
                        .map(|a| a.value.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            DotExpr::Variable(name) => write!(f, "{}", name.value),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Dot { lhs, rhs } => write!(f, "{}.{}", lhs.value, rhs.value),
            Expr::FunctionCall(name, vars) => write!(
                f,
                "{}({})",
                name.value,
                vars.iter()
                    .map(|v| format!("{}", v.value))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Binary(expr) => {
                write!(
                    f,
                    "{} {} {}",
                    expr.value.lhs.value, expr.value.op.value, expr.value.rhs.value
                )
            }
            Expr::Assignment(var, expr) => write!(f, "{} = {}", var.value, expr.value),
            Expr::Let(var, type_id, assign) => {
                write!(f, "let {}: {} = {}", var.value, type_id.value, assign.value)
            }
            Expr::Literal(literal) => write!(f, "{}", literal.value),
            Expr::StructLiteral(name, fields) => {
                write!(f, "{} {{", name.value)?;
                for (field_name, field_expr) in fields.iter() {
                    write!(f, "{}: {}, ", field_name.value, field_expr.value)?;
                }
                write!(f, "}}")
            }
            Expr::Variable(name) => write!(f, "{}", name.value),
            Expr::IfExpression {
                if_block: (if_cond, if_block),
                else_if_blocks,
                else_block,
            } => {
                write!(f, "if {} {}", if_cond.value, if_block.value)?;
                for (condition, block) in else_if_blocks {
                    write!(f, " else if {} {}", condition.value, block.value)?;
                }
                if let Some(else_block) = else_block {
                    write!(f, " else {}", else_block.value)?;
                }
                Ok(())
            }
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
            Expr::Loop(expr) => write!(f, "loop {}", expr.value),
            Expr::Return(expr) => write!(
                f,
                "return{}",
                expr.as_ref()
                    .map(|e| format!(" {}", e.value))
                    .unwrap_or(";".to_string())
            ),
            Expr::Break => write!(f, "break"),
            Expr::Continue => write!(f, "continue"),
        }
    }
}
