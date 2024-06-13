use expression::{parse_binary_expr, parse_expression, Expr};
use function::FunctionDeclaration;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{all_consuming, map},
    error::convert_error,
    multi::fold_many0,
    sequence::{pair, preceded, terminated},
    Parser as NomParser,
};

use crate::{error::Result, module::Module, tokenizer};

pub mod binary_expression;
pub mod expression;
pub mod function;
pub mod type_def;

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(self) -> Result<Module> {
        match parse_module().parse(self.input).map(|e| e.1) {
            Ok(res) => Ok(res),
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(
                crate::error::Error::new_invalid_token(convert_error(self.input, e)),
            ),
            Err(nom::Err::Incomplete(_)) => Err(crate::error::Error::new_unexpected_end_of_input(
                "Unexpected end of input".to_string(),
            )),
        }
    }

    pub fn parse_expression(self) -> Result<Expr> {
        match parse_expression.parse(self.input).map(|e| e.1) {
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

pub(self) fn parse_module<'a>(
) -> impl nom::Parser<&'a str, Module, nom::error::VerboseError<&'a str>> {
    all_consuming(fold_many0(
        parse_function_def(),
        || Module::new("Test"),
        |mut module, expr| {
            module.add_function(expr);
            module
        },
    ))
}

pub(self) fn parse_function_call<'a>(
) -> impl nom::Parser<&'a str, Expr, nom::error::VerboseError<&'a str>> {
    map(
        terminated(tokenizer::identifier(), pair(char('('), char(')'))),
        |ident| Expr::FunctionCall(ident.to_string()),
    )
}

fn parse_function_def<'a>(
) -> impl nom::Parser<&'a str, FunctionDeclaration, nom::error::VerboseError<&'a str>> {
    map(
        preceded(
            tokenizer::keyword_fn(),
            terminated(tokenizer::identifier(), pair(char('('), char(')'))),
        ),
        |ident| FunctionDeclaration {
            name: ident.to_string(),
        },
    )
}
