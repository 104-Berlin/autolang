use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, recognize},
    multi::{many0_count, many1},
    sequence::pair,
    IResult, Parser,
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

    /// Identifier
    /// ### Regex
    /// ```regex
    /// [a-zA-Z_][a-zA-Z0-9_]*
    /// ```
    Identifier(String),
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

        let (rest, result) = many1(alt((tokenize_identifier, tokenize_tags))).parse(self.input)?;

        assert!(rest.is_empty(), "Rest is not empty: {}", rest);

        tracing::info!("Finied tokenizing");

        Ok(result)
    }
}

/// Tokenizes an identifier
/// ### Regex
/// ```regex
/// [a-zA-Z_][a-zA-Z0-9_]*
/// ```
fn tokenize_identifier<'a>(input: &'a str) -> IResult<&'a str, Token> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| Token::Identifier(s.to_string()),
    )
    .parse(input)
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
fn tokenize_tags(input: &str) -> IResult<&str, Token> {
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
    ))(input)
}
