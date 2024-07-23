use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    /// User defined identifier (aka. variable names, function names, types, etc.)
    UserDefined(String),

    /// '('
    LParen,
    /// ')'
    RParen,
    /// '{'
    LBrace,
    /// '}'
    RBrace,
    /// '['
    LBracket,
    /// ']'
    RBracket,

    /// ':'
    Colon,
    /// '::'
    DoubleColon,
    /// ';'
    Semicolon,
    /// '.'
    Dot,
    /// ','
    Comma,

    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,
    /// '%'
    Modulus,

    /// '='
    Assignment,
    /// '=='
    Equals,
    /// '!='
    NotEquals,
    /// '>'
    GreaterThan,
    /// '>='
    GreaterThanOrEqual,
    /// '<'
    LessThan,
    /// '<='
    LessThanOrEqual,

    /// '&&'
    LogicalAnd,
    /// '||'
    LogicalOr,
    /// '!'
    LogicalNot,

    /// '->'
    Arrow,

    /// Built-in function
    /// 'fn'
    Function,
    /// Built-in keywords
    /// 'let'
    Let,
    /// 'true'
    True,
    /// 'false'
    False,

    /// Control flow
    /// 'if'
    If,
    /// 'else'
    Else,
    /// 'while'
    While,
    /// 'for'
    For,
    /// 'loop'
    Loop,
    /// 'return'
    Return,
    /// 'break'
    Break,
    /// 'continue'
    Continue,

    /// 'struct"
    Struct,
}

impl Identifier {
    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "loop" => Self::Loop,
            "for" => Self::For,
            "while" => Self::While,
            "return" => Self::Return,
            "break" => Self::Break,
            "continue" => Self::Continue,
            "struct" => Self::Struct,
            _ => Self::UserDefined(s),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::UserDefined(ident) => write!(f, "{}", ident),
            Identifier::LParen => write!(f, "("),
            Identifier::RParen => write!(f, ")"),
            Identifier::LBrace => write!(f, "{{"),
            Identifier::RBrace => write!(f, "}}"),
            Identifier::LBracket => write!(f, "["),
            Identifier::RBracket => write!(f, "]"),
            Identifier::Colon => write!(f, ":"),
            Identifier::DoubleColon => write!(f, "::"),
            Identifier::Semicolon => write!(f, ";"),
            Identifier::Dot => write!(f, "."),
            Identifier::Comma => write!(f, ","),
            Identifier::Plus => write!(f, "+"),
            Identifier::Minus => write!(f, "-"),
            Identifier::Star => write!(f, "*"),
            Identifier::Slash => write!(f, "/"),
            Identifier::Modulus => write!(f, "%"),
            Identifier::Assignment => write!(f, "="),
            Identifier::Equals => write!(f, "=="),
            Identifier::NotEquals => write!(f, "!="),
            Identifier::GreaterThan => write!(f, ">"),
            Identifier::GreaterThanOrEqual => write!(f, ">="),
            Identifier::LessThan => write!(f, "<"),
            Identifier::LessThanOrEqual => write!(f, "<="),
            Identifier::LogicalAnd => write!(f, "&&"),
            Identifier::LogicalOr => write!(f, "||"),
            Identifier::LogicalNot => write!(f, "!"),
            Identifier::Arrow => write!(f, "->"),
            Identifier::Function => write!(f, "fn"),
            Identifier::Let => write!(f, "let"),
            Identifier::True => write!(f, "true"),
            Identifier::False => write!(f, "false"),
            Identifier::If => write!(f, "if"),
            Identifier::Else => write!(f, "else"),
            Identifier::For => write!(f, "for"),
            Identifier::Loop => write!(f, "loop"),
            Identifier::While => write!(f, "while"),
            Identifier::Return => write!(f, "return"),
            Identifier::Break => write!(f, "break"),
            Identifier::Continue => write!(f, "continue"),
            Identifier::Struct => write!(f, "struct"),
        }
    }
}
