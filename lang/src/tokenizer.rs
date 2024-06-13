use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, one_of, space1},
    combinator::{map, map_res, opt, recognize},
    error::VerboseError,
    multi::{many0_count, many1},
    sequence::{delimited, pair, terminated, tuple},
    Parser,
};

use crate::parser::binary_expression::BinaryOperator;

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
pub fn identifier<'a>() -> impl Parser<&'a str, &'a str, VerboseError<&'a str>> {
    delimited(
        multispace0,
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        multispace0,
    )
}

pub fn binary_operator<'a>(
) -> impl Parser<&'a str, BinaryOperator, nom::error::VerboseError<&'a str>> {
    delimited(
        multispace0,
        alt((
            map(char('+'), |_| BinaryOperator::Add),
            map(char('-'), |_| BinaryOperator::Substract),
            map(char('*'), |_| BinaryOperator::Multiply),
            map(char('/'), |_| BinaryOperator::Divide),
        )),
        multispace0,
    )
}

pub fn keyword_fn<'a>() -> impl Parser<&'a str, &'a str, nom::error::VerboseError<&'a str>> {
    is_keyword("fn")
}

pub fn keyword_let<'a>() -> impl Parser<&'a str, &'a str, nom::error::VerboseError<&'a str>> {
    is_keyword("let")
}

/// Keywords need to have a space after
/// Use is_operator if you don't need a space after
pub fn is_keyword<'a>(
    keyword: &'a str,
) -> impl Parser<&'a str, &'a str, nom::error::VerboseError<&'a str>> {
    terminated(tag(keyword), space1)
}

pub fn numbers<'a>() -> impl Parser<&'a str, Literal, nom::error::VerboseError<&'a str>> {
    map(
        delimited(multispace0, integer(), multispace0),
        Literal::NumberInt,
    )
}

fn integer<'a>() -> impl Parser<&'a str, i64, nom::error::VerboseError<&'a str>> {
    map_res(
        tuple((opt(char('-')), recognize(many1(one_of("0123456789"))))),
        |(sign, digits): (Option<char>, &str)| {
            let sign = if sign.is_some() { -1 } else { 1 };
            digits.parse::<i64>().map(|n| sign * n)
        },
    )
}
