use std::iter::Peekable;

use identifier::Identifier;
use literal::Literal;
use source_span::{DefaultMetrics, Span};
use token::Token;

use crate::{error::Spanned, input_stream::InputStream};

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

    pub fn next_token(&mut self) -> Option<Spanned<Token>> {
        while let Some(c) = self.input.peek().filter(|c| c.is_whitespace()) {
            self.input.advance();
            self.span.push(c, &Self::METRICS);
        }

        let current_char = self.input.next()?;

        self.span = Span::from(self.span.end());

        self.span.push(current_char, &Self::METRICS);

        match current_char {
            '(' => Some(Spanned::new(
                Token::Identifier(Identifier::LParen),
                self.span,
            )),
            ')' => Some(Spanned::new(
                Token::Identifier(Identifier::RParen),
                self.span,
            )),
            '{' => Some(Spanned::new(
                Token::Identifier(Identifier::LBrace),
                self.span,
            )),
            '}' => Some(Spanned::new(
                Token::Identifier(Identifier::RBrace),
                self.span,
            )),
            '[' => Some(Spanned::new(
                Token::Identifier(Identifier::LBracket),
                self.span,
            )),
            ']' => Some(Spanned::new(
                Token::Identifier(Identifier::RBracket),
                self.span,
            )),
            ':' => Some(Spanned::new(
                Token::Identifier(Identifier::Colon),
                self.span,
            )),
            ';' => Some(Spanned::new(
                Token::Identifier(Identifier::Semicolon),
                self.span,
            )),
            '.' => Some(Spanned::new(Token::Identifier(Identifier::Dot), self.span)),
            ',' => Some(Spanned::new(
                Token::Identifier(Identifier::Comma),
                self.span,
            )),
            '+' => Some(Spanned::new(Token::Identifier(Identifier::Plus), self.span)),
            '-' => Some(Spanned::new(
                Token::Identifier(Identifier::Minus),
                self.span,
            )),
            '*' => Some(Spanned::new(Token::Identifier(Identifier::Star), self.span)),
            '/' => Some(Spanned::new(
                Token::Identifier(Identifier::Slash),
                self.span,
            )),
            '%' => Some(Spanned::new(
                Token::Identifier(Identifier::Modulus),
                self.span,
            )),
            '=' => Some(Spanned::new(
                Token::Identifier(Identifier::Assignment),
                self.span,
            )),
            '!' => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalNot),
                self.span,
            )),
            '&' => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalAnd),
                self.span,
            )),
            '|' => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalOr),
                self.span,
            )),
            '<' => Some(Spanned::new(
                Token::Identifier(Identifier::LessThan),
                self.span,
            )),
            '>' => Some(Spanned::new(
                Token::Identifier(Identifier::GreaterThan),
                self.span,
            )),
            c if c.is_numeric() => Some(self.parse_number_literal(current_char)),
            c if c.is_alphabetic() || c == '_' => Some(self.parse_identifier(current_char)),
            _ => None,
        }
    }

    fn parse_number_literal(&mut self, first_char: char) -> Spanned<Token> {
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
            Spanned::new(
                Token::Literal(Literal::NumberFloat(number.parse().unwrap())),
                self.span,
            )
        } else {
            Spanned::new(
                Token::Literal(Literal::NumberInt(number.parse().unwrap())),
                self.span,
            )
        }
    }

    fn parse_identifier(&mut self, first_char: char) -> Spanned<Token> {
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

        Spanned::new(
            Token::Identifier(Identifier::from_string(identifier)),
            self.span,
        )
    }
}

impl Iterator for Tokenizer {
    type Item = Spanned<Token>;

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
    type Output = Spanned<Token>;

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
