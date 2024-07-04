use std::fmt::Display;

use crate::{
    error::{Error, ErrorKind},
    spanned::Spanned,
    tokenizer::{identifier::Identifier, token::Token},
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
    pub lhs: Box<Spanned<Expr>>,
    pub op: Spanned<BinaryOperator>,
    pub rhs: Box<Spanned<Expr>>,
}

impl BinaryExpression {
    pub fn new(lhs: Spanned<Expr>, op: Spanned<BinaryOperator>, rhs: Spanned<Expr>) -> Self {
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
            BinaryOperator::Add | BinaryOperator::Substract => 100,
            BinaryOperator::Multiply | BinaryOperator::Divide => 200,
        }
    }
}

impl TryFrom<Spanned<Token>> for BinaryOperator {
    type Error = Error;

    fn try_from(Spanned::<Token> { value, span }: Spanned<Token>) -> Result<Self, Self::Error> {
        match value {
            Token::Identifier(Identifier::Plus) => Ok(BinaryOperator::Add),
            Token::Identifier(Identifier::Minus) => Ok(BinaryOperator::Substract),
            Token::Identifier(Identifier::Star) => Ok(BinaryOperator::Multiply),
            Token::Identifier(Identifier::Slash) => Ok(BinaryOperator::Divide),
            _ => Err(Error::new(span, ErrorKind::InvalidOperator)),
        }
    }
}

impl TryFrom<Spanned<Token>> for Spanned<BinaryOperator> {
    type Error = Error;

    fn try_from(token: Spanned<Token>) -> Result<Self, Self::Error> {
        let span = token.span;
        BinaryOperator::try_from(token).map(|op| Spanned::new(op, span))
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
