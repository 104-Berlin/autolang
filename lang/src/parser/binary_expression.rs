use std::fmt::Display;

use crate::{
    error::{Error, ErrorKind},
    tokenizer::{Identifier, Token, TokenKind},
};

use super::expression::Expr;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub lhs: Box<Expr>,
    pub op: BinaryOperator,
    pub rhs: Box<Expr>,
}

impl BinaryExpression {
    pub fn new(lhs: Expr, op: BinaryOperator, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl BinaryOperator {
    pub fn precedence(&self) -> i16 {
        match self {
            BinaryOperator::Add | BinaryOperator::Substract => 10,
            BinaryOperator::Multiply | BinaryOperator::Divide => 20,
        }
    }
}

impl TryFrom<Token> for BinaryOperator {
    type Error = Error;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.kind {
            TokenKind::Identifier(Identifier::Plus) => Ok(Self::Add),
            TokenKind::Identifier(Identifier::Minus) => Ok(Self::Substract),
            TokenKind::Identifier(Identifier::Star) => Ok(Self::Multiply),
            TokenKind::Identifier(Identifier::Slash) => Ok(Self::Divide),
            _ => Err(Error::new(value.span, ErrorKind::InvalidOperator)),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Substract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
        }
    }
}
