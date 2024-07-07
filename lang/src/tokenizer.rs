use std::iter::Peekable;

use identifier::Identifier;
use literal::Literal;
use source_span::{DefaultMetrics, Span};
use token::Token;

use crate::{input_stream::InputStream, spanned::Spanned};

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

        // Reset span, so it starts where it ends previously
        self.span = self.span.next();

        self.span.push(current_char, &Self::METRICS);

        match current_char {
            // '('
            '(' => Some(Spanned::new(
                Token::Identifier(Identifier::LParen),
                self.span,
            )),
            // ')'
            ')' => Some(Spanned::new(
                Token::Identifier(Identifier::RParen),
                self.span,
            )),
            // '{'
            '{' => Some(Spanned::new(
                Token::Identifier(Identifier::LBrace),
                self.span,
            )),
            // '}'
            '}' => Some(Spanned::new(
                Token::Identifier(Identifier::RBrace),
                self.span,
            )),
            // '['
            '[' => Some(Spanned::new(
                Token::Identifier(Identifier::LBracket),
                self.span,
            )),
            // ']'
            ']' => Some(Spanned::new(
                Token::Identifier(Identifier::RBracket),
                self.span,
            )),
            // '::'
            ':' if self.consume_checked(':').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::DoubleColon),
                self.span,
            )),
            // ':'
            ':' => Some(Spanned::new(
                Token::Identifier(Identifier::Colon),
                self.span,
            )),
            // ';'
            ';' => Some(Spanned::new(
                Token::Identifier(Identifier::Semicolon),
                self.span,
            )),
            // '.'
            '.' => Some(Spanned::new(Token::Identifier(Identifier::Dot), self.span)),
            // ','
            ',' => Some(Spanned::new(
                Token::Identifier(Identifier::Comma),
                self.span,
            )),
            // '+'
            '+' => Some(Spanned::new(Token::Identifier(Identifier::Plus), self.span)),
            // '->'
            '-' if self.consume_checked('>').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::Arrow),
                self.span,
            )),
            // '-'
            '-' => Some(Spanned::new(
                Token::Identifier(Identifier::Minus),
                self.span,
            )),
            // '*'
            '*' => Some(Spanned::new(Token::Identifier(Identifier::Star), self.span)),
            // '//'
            '/' if self.consume_checked('/').is_some() => {
                let _comment: String = self.consume_till("\n").into_iter().collect();
                self.next_token()
            }
            // '/*'
            '/' if self.consume_checked('*').is_some() => {
                let _comment: String = self.consume_till("*/").into_iter().collect();
                self.next_token()
            }
            // '/'
            '/' => Some(Spanned::new(
                Token::Identifier(Identifier::Slash),
                self.span,
            )),
            // '%'
            '%' => Some(Spanned::new(
                Token::Identifier(Identifier::Modulus),
                self.span,
            )),
            // '=='
            '=' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::Equals),
                self.span,
            )),
            // '='
            '=' => Some(Spanned::new(
                Token::Identifier(Identifier::Assignment),
                self.span,
            )),
            // '!='
            '!' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::NotEquals),
                self.span,
            )),
            // '!'
            '!' => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalNot),
                self.span,
            )),
            // '&&'
            '&' if self.consume_checked('&').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalAnd),
                self.span,
            )),
            // '||'
            '|' if self.consume_checked('|').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalOr),
                self.span,
            )),
            // '<='
            '<' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::LessThanOrEqual),
                self.span,
            )),
            // '<'
            '<' => Some(Spanned::new(
                Token::Identifier(Identifier::LessThan),
                self.span,
            )),
            // '>='
            '>' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::GreaterThanOrEqual),
                self.span,
            )),
            // '>'
            '>' => Some(Spanned::new(
                Token::Identifier(Identifier::GreaterThan),
                self.span,
            )),
            '\"' => Some(self.parse_string_literal()),
            c if c.is_numeric() => Some(self.parse_number_literal(current_char)),
            c if c.is_alphabetic() || c == '_' => Some(self.parse_identifier(current_char)),
            _ => None,
        }
    }

    fn parse_string_literal(&mut self) -> Spanned<Token> {
        let mut string = String::new();

        while let Some(c) = self.input.next() {
            self.span.push(c, &Self::METRICS);
            if c == '"' {
                break;
            } else if c == '\\' {
                let Some(next) = self.input.next() else {
                    break;
                };

                self.span.push(next, &Self::METRICS);

                if next == '\"' {
                    string.push('\"');
                }
            } else {
                string.push(c);
            }
        }

        Spanned::new(Token::Literal(Literal::String(string)), self.span)
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

impl Tokenizer {
    fn consume_checked(&mut self, expected: char) -> Option<char> {
        self.input.consume_checked(expected).inspect(|c| {
            self.span.push(*c, &Self::METRICS);
        })
    }

    fn consume_till(&mut self, expected: &str) -> Option<String> {
        assert!(!expected.is_empty(), "expected must not be empty");
        let mut buffer = String::new();

        let expected = expected.chars().collect::<Vec<_>>();
        let expected = expected.as_slice();

        while let Some(c) = self.input.next() {
            self.span.push(c, &Self::METRICS);
            if c == expected[0] {
                let mut found = true;
                for e in &expected[1..] {
                    let next_input = self.input.next()?;
                    self.span.push(next_input, &Self::METRICS);

                    if next_input != *e {
                        found = false;
                        break;
                    }
                }

                if found {
                    break;
                }
            }

            buffer.push(c);
        }
        Some(buffer)
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
