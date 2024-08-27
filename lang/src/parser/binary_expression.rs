use std::fmt::Display;

use miette::{miette, Error, LabeledSpan};

use crate::{
    spanned::Spanned,
    tokenizer::{identifier::Identifier, token::Token},
};

use super::expression::Expr;

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Assign,

    Add,
    Substract,
    Multiply,
    Divide,

    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
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
            BinaryOperator::Assign => 1,
            BinaryOperator::Add | BinaryOperator::Substract => 100,
            BinaryOperator::Multiply | BinaryOperator::Divide => 200,
            BinaryOperator::And => 10,
            BinaryOperator::Or => 10,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 5,
            BinaryOperator::LessThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqual => 5,
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
            Token::Identifier(Identifier::LogicalAnd) => Ok(BinaryOperator::And),
            Token::Identifier(Identifier::LogicalOr) => Ok(BinaryOperator::Or),
            Token::Identifier(Identifier::Equals) => Ok(BinaryOperator::Equal),
            Token::Identifier(Identifier::NotEquals) => Ok(BinaryOperator::NotEqual),
            Token::Identifier(Identifier::LessThan) => Ok(BinaryOperator::LessThan),
            Token::Identifier(Identifier::LessThanOrEqual) => Ok(BinaryOperator::LessThanOrEqual),
            Token::Identifier(Identifier::GreaterThan) => Ok(BinaryOperator::GreaterThan),
            Token::Identifier(Identifier::GreaterThanOrEqual) => {
                Ok(BinaryOperator::GreaterThanOrEqual)
            }
            Token::Identifier(Identifier::Assignment) => Ok(BinaryOperator::Assign),
            _ => Err(miette!(
                labels = [LabeledSpan::at(span, "here")],
                "Invalid binary operator"
            )),
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
            BinaryOperator::Assign => write!(f, "="),
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Substract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}
