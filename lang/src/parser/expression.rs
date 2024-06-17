use std::fmt::Display;

use nom::{branch::alt, combinator::map, multi::many0, sequence::tuple, Parser};

use crate::tokenizer::{self, Literal};

use super::{
    binary_expression::BinaryExpression,
    parse_function_call,
    spans::{InputSpan, NomResult},
};

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(String),
    Binary(BinaryExpression),

    Literal(Literal),
    Variable(String),
}

pub fn parse_expression(input: InputSpan) -> NomResult<'_, Expr> {
    alt((parse_binary_expr, parse_primary_expr))(input)
}

pub fn parse_primary_expr(input: InputSpan) -> NomResult<'_, Expr> {
    alt((
        map(tokenizer::numbers, Expr::Literal),
        map(tokenizer::identifier, |name| {
            Expr::Variable(name.to_string())
        }),
        parse_function_call,
    ))(input)
}

pub fn parse_binary_expr<'a>(input: InputSpan) -> NomResult<'_, Expr> {
    let mut binary_parser = tuple((
        parse_primary_expr,
        many0(tuple((tokenizer::binary_operator, parse_primary_expr))),
    ));

    let (input, (first, rest)) = binary_parser.parse(input)?;
    //let folded = fold_binary_expr(first, rest).map_err()?;
    Ok((input, first))
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FunctionCall(name) => write!(f, "{}()", name),
            Expr::Binary(expr) => {
                write!(f, "({} {} {})", expr.left, expr.operator, expr.right)
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
