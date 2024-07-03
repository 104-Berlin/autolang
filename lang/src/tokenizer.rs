use std::{fmt::Display, iter::Peekable};

use identifier::Identifier;
use literal::Literal;
use source_span::{DefaultMetrics, Span};
use token::{Token, TokenKind};

use crate::input_stream::InputStream;

pub mod identifier;
pub mod literal;
pub mod token;

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

        let current_char = self.input.next()?;

        self.span = Span::from(self.span.end());

        self.span.push(current_char, &Self::METRICS);

        match current_char {
            '(' => Some(Token::new(
                TokenKind::Identifier(Identifier::LParen),
                self.span,
            )),
            ')' => Some(Token::new(
                TokenKind::Identifier(Identifier::RParen),
                self.span,
            )),
            '{' => Some(Token::new(
                TokenKind::Identifier(Identifier::LBrace),
                self.span,
            )),
            '}' => Some(Token::new(
                TokenKind::Identifier(Identifier::RBrace),
                self.span,
            )),
            '[' => Some(Token::new(
                TokenKind::Identifier(Identifier::LBracket),
                self.span,
            )),
            ']' => Some(Token::new(
                TokenKind::Identifier(Identifier::RBracket),
                self.span,
            )),
            ':' => Some(Token::new(
                TokenKind::Identifier(Identifier::Colon),
                self.span,
            )),
            ';' => Some(Token::new(
                TokenKind::Identifier(Identifier::Semicolon),
                self.span,
            )),
            '.' => Some(Token::new(
                TokenKind::Identifier(Identifier::Dot),
                self.span,
            )),
            ',' => Some(Token::new(
                TokenKind::Identifier(Identifier::Comma),
                self.span,
            )),
            '+' => Some(Token::new(
                TokenKind::Identifier(Identifier::Plus),
                self.span,
            )),
            '-' => Some(Token::new(
                TokenKind::Identifier(Identifier::Minus),
                self.span,
            )),
            '*' => Some(Token::new(
                TokenKind::Identifier(Identifier::Star),
                self.span,
            )),
            '/' => Some(Token::new(
                TokenKind::Identifier(Identifier::Slash),
                self.span,
            )),
            '%' => Some(Token::new(
                TokenKind::Identifier(Identifier::Modulus),
                self.span,
            )),
            '=' => Some(Token::new(
                TokenKind::Identifier(Identifier::Assignment),
                self.span,
            )),
            '!' => Some(Token::new(
                TokenKind::Identifier(Identifier::LogicalNot),
                self.span,
            )),
            '&' => Some(Token::new(
                TokenKind::Identifier(Identifier::LogicalAnd),
                self.span,
            )),
            '|' => Some(Token::new(
                TokenKind::Identifier(Identifier::LogicalOr),
                self.span,
            )),
            '<' => Some(Token::new(
                TokenKind::Identifier(Identifier::LessThan),
                self.span,
            )),
            '>' => Some(Token::new(
                TokenKind::Identifier(Identifier::GreaterThan),
                self.span,
            )),
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
            Token::new(
                TokenKind::Literal(Literal::NumberFloat(number.parse().unwrap())),
                self.span,
            )
        } else {
            Token::new(
                TokenKind::Literal(Literal::NumberInt(number.parse().unwrap())),
                self.span,
            )
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

        Token::new(
            TokenKind::Identifier(Identifier::from_string(identifier)),
            self.span,
        )
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier(identifier) => write!(f, "{}", identifier),
            TokenKind::Literal(literal) => write!(f, "{}", literal),
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
            Identifier::Function => write!(f, "fn"),
            Identifier::Let => write!(f, "let"),
            Identifier::True => write!(f, "true"),
            Identifier::False => write!(f, "false"),
            Identifier::If => write!(f, "if"),
            Identifier::Else => write!(f, "else"),
            Identifier::While => write!(f, "while"),
            Identifier::Return => write!(f, "return"),
            Identifier::Break => write!(f, "break"),
            Identifier::Continue => write!(f, "continue"),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::NumberInt(num) => write!(f, "{}", num),
            Literal::NumberFloat(num) => write!(f, "{}", num),
        }
    }
}
