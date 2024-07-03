use std::{
    cell::{Ref, RefCell},
    iter::Peekable,
    ops::DerefMut,
};

use source_span::{DefaultMetrics, Span};

use crate::input_stream::InputStream;

/// Literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Number literal
    NumberInt(i64),
    NumberFloat(f64),
    // String
    // String(String),
}

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
    fn from_string(s: String) -> Self {
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

/// Tokens
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Identifier
    Identifier(Identifier),
    /// Literal
    Literal(Literal),
}

pub struct Tokenizer {
    input: Box<dyn InputStream<Output = char>>,
    span: Span,
}

impl Tokenizer {
    pub const METRICS: source_span::DefaultMetrics = DefaultMetrics::new();

    pub fn new(input: impl InputStream<Output = char> + 'static) -> Self {
        Self {
            input: Box::new(input),
            span: Span::default(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.input.peek().filter(|c| c.is_whitespace()) {
            self.input.advance();
            self.span.push(c, &Self::METRICS);
        }

        let Some(current_char) = self.input.next() else {
            return None;
        };

        self.span = Span::from(self.span.end());
        
        self.span.push(current_char, &Self::METRICS);


        match current_char {
            '(' => Some(Token::new(TokenKind::Identifier(Identifier::LParen), self.span)),
            ')' => Some(Token::new(TokenKind::Identifier(Identifier::RParen), self.span)),
            '{' => Some(Token::new(TokenKind::Identifier(Identifier::LBrace), self.span)),
            '}' => Some(Token::new(TokenKind::Identifier(Identifier::RBrace), self.span)),
            '[' => Some(Token::new(TokenKind::Identifier(Identifier::LBracket), self.span)),
            ']' => Some(Token::new(TokenKind::Identifier(Identifier::RBracket), self.span)),
            ':' => Some(Token::new(TokenKind::Identifier(Identifier::Colon), self.span)),
            ';' => Some(Token::new(TokenKind::Identifier(Identifier::Semicolon), self.span)),
            '.' => Some(Token::new(TokenKind::Identifier(Identifier::Dot), self.span)),
            ',' => Some(Token::new(TokenKind::Identifier(Identifier::Comma), self.span)),
            '+' => Some(Token::new(TokenKind::Identifier(Identifier::Plus), self.span)),
            '-' => Some(Token::new(TokenKind::Identifier(Identifier::Minus), self.span)),
            '*' => Some(Token::new(TokenKind::Identifier(Identifier::Star), self.span)),
            '/' => Some(Token::new(TokenKind::Identifier(Identifier::Slash), self.span)),
            '%' => Some(Token::new(TokenKind::Identifier(Identifier::Modulus), self.span)),
            '=' => Some(Token::new(TokenKind::Identifier(Identifier::Assignment), self.span)),
            '!' => Some(Token::new(TokenKind::Identifier(Identifier::LogicalNot), self.span)),
            '&' => Some(Token::new(TokenKind::Identifier(Identifier::LogicalAnd), self.span)),
            '|' => Some(Token::new(TokenKind::Identifier(Identifier::LogicalOr), self.span)),
            '<' => Some(Token::new(TokenKind::Identifier(Identifier::LessThan), self.span)),
            '>' => Some(Token::new(TokenKind::Identifier(Identifier::GreaterThan), self.span)),
            c if c.is_numeric() => Some(self.parse_number_literal(current_char)),
            c if c.is_alphabetic() || c == '_' => Some(self.parse_identifier(current_char)),
            _ => None,
        }
    }

    fn parse_number_literal(&mut self, first_char: char) -> Token {
        let mut number = String::new();
        number.push(first_char);

        while let Some(c) = self.input.peek() {
            if c.is_numeric() || c == '.' {
                number.push(c);
                self.span.push(c, &Self::METRICS);
                self.input.advance();
            } else {
                break;
            }
        }

        if number.contains('.') {
            Token::new(TokenKind::Literal(Literal::NumberFloat(number.parse().unwrap())), self.span)
        } else {
            Token::new(TokenKind::Literal(Literal::NumberInt(number.parse().unwrap())), self.span)
        }
    }

    fn parse_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);

        while let Some(c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.span.push(c, &Self::METRICS);
                self.input.advance();
            } else {
                break;
            }
        }

        Token::new(TokenKind::Identifier(Identifier::from_string(identifier)), self.span)
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub struct TokenizerStream {
    tokenizer: Peekable<Tokenizer>,
}

impl TokenizerStream {
    pub fn new(input: impl InputStream<Output = char> + 'static) -> Self {
        Self {
            tokenizer: Tokenizer::new(input).peekable(),
        }
    }
}

impl InputStream for TokenizerStream {
    type Output = Token;

    fn next(&mut self) -> Option<Self::Output> {
        self.tokenizer.next()
    }

    fn peek(&mut self) -> Option<Self::Output> {
        self.tokenizer.peek().cloned()
    }

    fn advance(&mut self) {
        self.tokenizer.next();
    }
}

