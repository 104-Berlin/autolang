use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of, space0, space1},
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::{context, convert_error, VerboseError},
    multi::{many0_count, many1},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::error::Result;

/// Tokens of the language
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Number literal
    NumberInt(i64),
    NumberFloat(f64),

    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,

    /// '('
    LParen,
    /// ')'
    RParen,
    /// '{'
    LBrace,
    /// '}'
    RBrace,
    /// '['
    LBracket,
    /// ']'
    RBracket,
    /// '='
    Assign,

    /// Identifier
    /// ### Regex
    /// ```regex
    /// [a-zA-Z_][a-zA-Z0-9_]*
    /// ```
    Identifier(String),

    /// Special identifier

    /// "fn"
    Fn,
    /// "let"
    Let,
}

pub struct Tokenizer<'a> {
    input: &'a str,
}

impl Tokenizer<'_> {
    /// Creates a new tokenizer to generate tokens from the input
    /// # Example
    /// ```
    /// use lang::tokenizer::Tokenizer;
    /// let tokens = Tokenizer::new("some_identifier").tokenize();
    /// ```
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer { input }
    }

    pub fn tokenize(self) -> Result<Vec<Token>> {
        tracing::info!("Tokenizing input: {:.24}", self.input);

        let result = tokenize_all::<'_>().parse(self.input);
        match result {
            Ok((_, tokens)) => Ok(tokens),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(
                crate::error::Error::new_invalid_token(convert_error(self.input, e)),
            ),
            Err(nom::Err::Incomplete(_)) => Err(crate::error::Error::new_unexpected_end_of_input(
                "Unexpected end of input".to_string(),
            )),
        }
    }
}

fn tokenize_all<'a>() -> impl Parser<&'a str, Vec<Token>, nom::error::VerboseError<&'a str>> {
    preceded(
        space0,
        context(
            "ALL",
            all_consuming(many1(context(
                "New Token",
                terminated(
                    alt((
                        tokenize_special_identifier(),
                        tokenize_tags(),
                        tokenize_identifier(),
                        tokenize_numbers(),
                    )),
                    space0,
                ),
            ))),
        ),
    )
}

/// Tokenizes an identifier
/// ### Regex
/// ```regex
/// [a-zA-Z_][a-zA-Z0-9_]*
/// ```
fn tokenize_identifier<'a>() -> impl Parser<&'a str, Token, VerboseError<&'a str>> {
    context(
        "Identifier",
        map(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0_count(alt((alphanumeric1, tag("_")))),
            )),
            |s: &str| Token::Identifier(s.to_string()),
        ),
    )
}

/// - '+' -> Token::Plus
/// - '-' -> Token::Minus
/// - '*' -> Token::Star
/// - '/' -> Token::Slash
/// - '(' -> Token::LParen
/// - ')' -> Token::RParen
/// - '{' -> Token::LBrace
/// - '}' -> Token::RBrace
/// - '[' -> Token::LBracket
/// - ']' -> Token::RBracket
fn tokenize_tags<'a>() -> impl Parser<&'a str, Token, nom::error::VerboseError<&'a str>> {
    context(
        "Tags",
        alt((
            map(tag("+"), |_| Token::Plus),
            map(tag("-"), |_| Token::Minus),
            map(tag("*"), |_| Token::Star),
            map(tag("/"), |_| Token::Slash),
            map(tag("("), |_| Token::LParen),
            map(tag(")"), |_| Token::RParen),
            map(tag("{"), |_| Token::LBrace),
            map(tag("}"), |_| Token::RBrace),
            map(tag("["), |_| Token::LBracket),
            map(tag("]"), |_| Token::RBracket),
            map(tag("="), |_| Token::Assign),
        )),
    )
}

/// - "fn" -> Token::Fn
/// - "let" -> Token::Let
fn tokenize_special_identifier<'a>(
) -> impl Parser<&'a str, Token, nom::error::VerboseError<&'a str>> {
    context(
        "Speziel Identifier",
        terminated(
            alt((
                map(tag("fn"), |_| Token::Fn),
                map(tag("let"), |_| Token::Let),
            )),
            space1,
        ),
    )
}

fn tokenize_numbers<'a>() -> impl Parser<&'a str, Token, nom::error::VerboseError<&'a str>> {
    context("Number", map(integer(), |num| Token::NumberInt(num)))
}

/*fn integer<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, i64, E> {
    let a = combinator::map_res(
        tuple((
            opt(nom::character::complete::char::<_, E>('-')),
            recognize(many1(one_of("1234567890"))),
        )),
        |(sign, digit)| {
            let sign = match sign {
                Some(_) => -1,
                None => 1,
            };

            digit.parse::<i64>().map(|n| n * sign)
        },
    )(input);

    todo!()
}*/
fn integer<'a>() -> impl Parser<&'a str, i64, nom::error::VerboseError<&'a str>> {
    context(
        "",
        map_res(
            tuple((opt(char('-')), recognize(many1(one_of("0123456789"))))),
            |(sign, digits): (Option<char>, &str)| {
                let sign = if sign.is_some() { -1 } else { 1 };
                digits.parse::<i64>().map(|n| sign * n)
            },
        ),
    )
}
