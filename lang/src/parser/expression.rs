use std::fmt::Display;

use nom::{branch::alt, combinator::{map, peek}, error::ErrorKind, sequence::tuple, IResult, Parser};

use crate::tokenizer::{self, Literal};

use super::{binary_expression::BinaryExpression, parse_function_call};

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(String),
    Binary(BinaryExpression),

    Literal(Literal),
    Variable(String),
}

pub struct ExpressionParser;

impl ExpressionParser {
    fn parse<'a>(
        &mut self,
        input: &'a str,
    ) -> IResult<&'a str, Expr, nom::error::VerboseError<&'a str>> {
        parse_expression(input)
    }
}

pub fn parse_expression(input: &str) -> IResult<&str, Expr, nom::error::VerboseError<&str>> {
    let (input, primary_expr) = parse_primary_expr(input)?;
    if peek(tokenizer::binary_operator())(input).is_err() {
        return Ok((input, primary_expr));
    }
    parse_binary_expr(0, primary_expr).parse(input)
}

pub fn parse_primary_expr(input: &str) -> IResult<&str, Expr, nom::error::VerboseError<&str>> {
    alt((
        map(tokenizer::numbers(), Expr::Literal),
        map(tokenizer::identifier(), |name| {
            Expr::Variable(name.to_string())
        }),
        parse_function_call(),
    ))
    .parse(input)
}

pub fn parse_binary_expr<'a>(last_precedence: i16, mut lhs: Expr) -> impl FnMut(&str) -> IResult<&str, Expr, nom::error::VerboseError<&str>> {
    move |mut input| {
        let (input, operator) = tokenizer::binary_operator().parse(input)?;
        let precedence = operator.precedence();
        if precedence > last_precedence {
            return Ok((input, lhs.clone()));
        }
        let (input, rhs) = parse_expression(input)?;
    
        Ok((
            input,
            Expr::Binary(BinaryExpression::new(lhs.clone(), operator, rhs)),
        ))
    }
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
