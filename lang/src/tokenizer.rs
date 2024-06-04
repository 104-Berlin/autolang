use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of, space1},
    combinator::{map, map_res, opt, recognize},
    error::VerboseError,
    multi::{many0_count, many1},
    sequence::{pair, terminated, tuple},
    Parser,
};

/// Literals
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
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
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
    map(integer(), Literal::NumberInt)
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
