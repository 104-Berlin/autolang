use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, one_of, space1},
    combinator::{map, map_res, opt, recognize},
    multi::{many0_count, many1},
    sequence::{delimited, pair, terminated, tuple},
};

use crate::parser::{
    binary_expression::BinaryOperator,
    spans::{InputSpan, NomResult},
};

/// Literals
#[derive(Debug, Clone)]
pub enum Literal {
    /// Number literal
    NumberInt(i64),
    NumberFloat(f64),
    // String
    // String(String),
}

/// Tokenizes an identifier
/// ### Regex
/// ```regex
/// [a-zA-Z_][a-zA-Z0-9_]*
/// ```
pub fn identifier<'a>(input: InputSpan<'_>) -> NomResult<InputSpan<'_>> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn binary_operator<'a>(input: InputSpan<'a>) -> NomResult<BinaryOperator> {
    alt((
        map(char('+'), |_| BinaryOperator::Add),
        map(char('-'), |_| BinaryOperator::Substract),
        map(char('*'), |_| BinaryOperator::Multiply),
        map(char('/'), |_| BinaryOperator::Divide),
    ))(input)
}

pub fn keyword_fn(input: InputSpan) -> NomResult<InputSpan> {
    keyword("fn")(input)
}

pub fn keyword_let(input: InputSpan) -> NomResult<InputSpan> {
    keyword("let")(input)
}

/// Keywords need to have a space after
/// Use is_operator if you don't need a space after
pub fn keyword(keyword: &str) -> impl FnMut(InputSpan) -> NomResult<'_, InputSpan> + '_ {
    move |input: InputSpan| terminated(tag(keyword), space1)(input)
}

pub fn numbers<'a>(input: InputSpan) -> NomResult<Literal> {
    map(
        delimited(multispace0, integer, multispace0),
        Literal::NumberInt,
    )(input)
}

fn integer(input: InputSpan) -> NomResult<'_, i64> {
    map_res(
        tuple((opt(char('-')), recognize(many1(one_of("0123456789"))))),
        |(sign, digits): (Option<char>, InputSpan)| {
            let sign = if sign.is_some() { -1 } else { 1 };
            digits.parse::<i64>().map(|n| sign * n)
        },
    )(input)
}
