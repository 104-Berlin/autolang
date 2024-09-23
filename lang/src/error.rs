use std::fmt::Display;

use miette::{miette, Diagnostic, SourceSpan};
use thiserror::Error;
use virtual_machine::error::VMError;

use crate::{
    parser::{binary_expression::BinaryOperator, type_def::TypeID},
    tokenizer::token::Token,
};

pub trait VMToMietteError<T> {
    fn to_miette_error(self) -> Result<T, miette::Error>;
}

impl<T> VMToMietteError<T> for Result<T, VMError> {
    fn to_miette_error(self) -> Result<T, miette::Error> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(miette!("{}", err)),
        }
    }
}

#[derive(Debug)]
pub enum ExpectedToken {
    Token(Token),
    None(String),
}

impl<S: Into<String>> From<S> for ExpectedToken {
    fn from(s: S) -> Self {
        Self::None(s.into())
    }
}

impl From<Token> for ExpectedToken {
    fn from(token: Token) -> Self {
        Self::Token(token)
    }
}

impl Display for ExpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Token(token) => write!(f, "expected '{}'", token),
            Self::None(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Unexpected token '{found}'")]

pub struct UnexpectedToken {
    pub found: Token,

    #[label("{expected}")]
    pub span: SourceSpan,

    pub expected: ExpectedToken,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Invalid number of arguments: found {found}, expected {expected}")]
pub struct InvalidNumberOfArguments {
    pub found: usize,
    pub expected: usize,

    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Invalid number of arguments: found {found}, expected {expected}")]
pub struct TypeMismatch {
    pub found: TypeID,
    pub expected: TypeID,

    #[source]
    #[diagnostic_source]
    pub reason: TypeMismatchReason,

    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
pub enum TypeMismatchReason {
    #[error("Function return type")]
    FunctionReturn,
    #[error("Function argument type")]
    FunctionArgument,
    #[error("Binary operation {0}")]
    BinaryOperation(BinaryOperator),
    #[error("Variable assignment")]
    VariableAssignment,
}

#[derive(Error, Debug, Diagnostic)]
pub enum ControllFlow {
    #[error("Continue statement outside of loop")]
    Continue,
    #[error("Break statement outside of loop")]
    Break,
    #[error("Return statement outside of function")]
    Return,
}
