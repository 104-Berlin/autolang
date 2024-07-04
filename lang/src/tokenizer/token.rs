use std::fmt::Display;

use super::{identifier::Identifier, literal::Literal};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Identifier
    Identifier(Identifier),
    /// Literal
    Literal(Literal),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Literal(literal) => write!(f, "{}", literal),
        }
    }
}
