use expression::{parse_expression as p_expression, Expr};
use function::FunctionDeclaration;
use nom::{
    character::complete::char,
    combinator::{all_consuming, map},
    multi::fold_many0,
    sequence::{pair, preceded, terminated},
};
use spans::{with_span, InputSpan, NomResult, Spanned};

use crate::{
    error::{Error, ErrorKind, Result},
    module::Module,
    tokenizer,
};

pub mod binary_expression;
pub mod expression;
pub mod function;
pub mod helpers;
pub mod spans;
pub mod type_def;

pub fn parse_module(input: InputSpan) -> Result<Module> {
    match module_parser(input) {
        Ok((_, module)) => Ok(module),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(Error::new(input, ErrorKind::UnexpectedEOF)),
    }
}

pub fn parse_expression(input: InputSpan) -> Result<Expr> {
    match p_expression(input).map(|e| e.1) {
        Ok(res) => Ok(res.extra),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(Error::new(input, ErrorKind::UnexpectedEOF)),
    }
}

pub(self) fn module_parser<'a>(input: InputSpan) -> NomResult<Module> {
    all_consuming(fold_many0(
        parse_function_def,
        || Module::new("Test"),
        |mut module, expr| {
            module.add_function(expr.extra);
            module
        },
    ))(input)
}

pub(self) fn parse_function_call<'a>(input: InputSpan) -> NomResult<Spanned<Expr>> {
    with_span(map(
        terminated(tokenizer::identifier, pair(char('('), char(')'))),
        |ident| Expr::FunctionCall(ident.to_string()),
    ))(input)
}

fn parse_function_def<'a>(input: InputSpan) -> NomResult<'_, Spanned<FunctionDeclaration>> {
    with_span(map(
        preceded(
            tokenizer::keyword_fn,
            terminated(tokenizer::identifier, pair(char('('), char(')'))),
        ),
        |ident| FunctionDeclaration {
            name: ident.to_string(),
        },
    ))(input)
}
