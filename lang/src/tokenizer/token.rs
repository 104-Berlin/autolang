use source_span::Span;

use super::{identifier::Identifier, literal::Literal};

/// Tokens
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Identifier
    Identifier(Identifier),
    /// Literal
    Literal(Literal),
}
