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
    /// 'return'
    Return,
    /// 'break'
    Break,
    /// 'continue'
    Continue,
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
            "while" => Self::While,
            "return" => Self::Return,
            "break" => Self::Break,
            "continue" => Self::Continue,
            _ => Self::UserDefined(s),
        }
    }
}
