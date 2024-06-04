use nom::{
    branch::alt,
    character::complete::char,
    combinator::map,
    error::convert_error,
    sequence::{pair, preceded, terminated},
    Parser as NomParser,
};

use crate::{error::Result, tokenizer};

pub enum Expr {
    FunctionCall(String),
    FunctionDef(String),
}

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(self) -> Result<Expr> {
        match parse_all().parse(self.input).map(|e| e.1) {
            Ok(res) => Ok(res),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(
                crate::error::Error::new_invalid_token(convert_error(self.input, e)),
            ),
            Err(nom::Err::Incomplete(_)) => Err(crate::error::Error::new_unexpected_end_of_input(
                "Unexpected end of input".to_string(),
            )),
        }
    }
}

fn parse_all<'a>() -> impl nom::Parser<&'a str, Expr, nom::error::VerboseError<&'a str>> {
    alt((parse_function_call(), parse_function_def()))
}

fn parse_function_call<'a>() -> impl nom::Parser<&'a str, Expr, nom::error::VerboseError<&'a str>> {
    map(
        terminated(tokenizer::identifier(), pair(char('('), char(')'))),
        |ident| Expr::FunctionCall(ident.to_string()),
    )
}

fn parse_function_def<'a>() -> impl nom::Parser<&'a str, Expr, nom::error::VerboseError<&'a str>> {
    map(
        preceded(
            tokenizer::keyword_fn(),
            terminated(tokenizer::identifier(), pair(char('('), char(')'))),
        ),
        |ident| Expr::FunctionDef(ident.to_string()),
    )
}
