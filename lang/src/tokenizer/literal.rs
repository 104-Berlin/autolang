use std::fmt::Display;

/// Literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Number literal
    NumberInt(i64),
    NumberFloat(f64),
    String(String),
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::NumberInt(num) => write!(f, "{}", num),
            Literal::NumberFloat(num) => write!(f, "{}", num),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}
