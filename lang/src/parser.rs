use expression::{parse_expression, Expr};
use function::FunctionDeclaration;
use nom::{
    character::complete::char,
    combinator::{all_consuming, map},
    multi::fold_many0,
    sequence::{pair, preceded, terminated},
};
use spans::{InputSpan, NomResult};

use crate::{
    error::{Error, ErrorKind, Result},
    module::Module,
    tokenizer,
};

pub mod binary_expression;
pub mod expression;
pub mod function;
pub mod spans;
pub mod type_def;

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(self) -> Result<Module> {
        let input = InputSpan::new(self.input);
        match parse_module(input).map(|e| e.1) {
            Ok(res) => Ok(res),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
            Err(nom::Err::Incomplete(_)) => Err(Error::new(input, ErrorKind::UnexpectedEOF)),
        }
    }

    pub fn parse_expression(self) -> Result<Expr> {
        let input = InputSpan::new(self.input);
        match parse_expression(input).map(|e| e.1) {
            Ok(res) => Ok(res),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
            Err(nom::Err::Incomplete(_)) => Err(Error::new(input, ErrorKind::UnexpectedEOF)),
        }
    }
}

pub(self) fn parse_module<'a>(input: InputSpan) -> NomResult<Module> {
    all_consuming(fold_many0(
        parse_function_def,
        || Module::new("Test"),
        |mut module, expr| {
            module.add_function(expr);
            module
        },
    ))(input)
}

pub(self) fn parse_function_call<'a>(input: InputSpan) -> NomResult<Expr> {
    map(
        terminated(tokenizer::identifier, pair(char('('), char(')'))),
        |ident| Expr::FunctionCall(ident.to_string()),
    )(input)
}

fn parse_function_def<'a>(input: InputSpan) -> NomResult<'_, FunctionDeclaration> {
    map(
        preceded(
            tokenizer::keyword_fn,
            terminated(tokenizer::identifier, pair(char('('), char(')'))),
        ),
        |ident| FunctionDeclaration {
            name: ident.to_string(),
        },
    )(input)
}
