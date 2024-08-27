use std::iter::Peekable;

use identifier::Identifier;
use literal::Literal;
use token::Token;

use crate::{input_stream::InputStream, spanned::Spanned};

pub mod identifier;
pub mod literal;
pub mod token;

pub struct Tokenizer<'a> {
    input: Box<dyn InputStream<Output = char> + 'a>,
    offset: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: impl InputStream<Output = char> + 'a) -> Self {
        Self {
            input: Box::new(input),
            offset: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<Token>> {
        while let Some(c) = self.input.peek().filter(|c| c.is_whitespace()) {
            self.offset += c.len_utf8();
            self.input.advance();
        }

        let current_char = self.input.next()?;

        let start_offset = self.offset;
        self.offset += current_char.len_utf8();

        match current_char {
            // '('
            '(' => Some(Spanned::new(
                Token::Identifier(Identifier::LParen),
                (start_offset, 1).into(),
            )),
            // ')'
            ')' => Some(Spanned::new(
                Token::Identifier(Identifier::RParen),
                (start_offset, 1).into(),
            )),
            // '{'
            '{' => Some(Spanned::new(
                Token::Identifier(Identifier::LBrace),
                (start_offset, 1).into(),
            )),
            // '}'
            '}' => Some(Spanned::new(
                Token::Identifier(Identifier::RBrace),
                (start_offset, 1).into(),
            )),
            // '['
            '[' => Some(Spanned::new(
                Token::Identifier(Identifier::LBracket),
                (start_offset, 1).into(),
            )),
            // ']'
            ']' => Some(Spanned::new(
                Token::Identifier(Identifier::RBracket),
                (start_offset, 1).into(),
            )),
            // '::'
            ':' if self.consume_checked(':').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::DoubleColon),
                (start_offset, 1).into(),
            )),
            // ':'
            ':' => Some(Spanned::new(
                Token::Identifier(Identifier::Colon),
                (start_offset, 1).into(),
            )),
            // ';'
            ';' => Some(Spanned::new(
                Token::Identifier(Identifier::Semicolon),
                (start_offset, 1).into(),
            )),
            // '.'
            '.' => Some(Spanned::new(
                Token::Identifier(Identifier::Dot),
                (start_offset, 1).into(),
            )),
            // ','
            ',' => Some(Spanned::new(
                Token::Identifier(Identifier::Comma),
                (start_offset, 1).into(),
            )),
            // '+'
            '+' => Some(Spanned::new(
                Token::Identifier(Identifier::Plus),
                (start_offset, 1).into(),
            )),
            // '->'
            '-' if self.consume_checked('>').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::Arrow),
                (start_offset, 1).into(),
            )),
            // '-'
            '-' => Some(Spanned::new(
                Token::Identifier(Identifier::Minus),
                (start_offset, 1).into(),
            )),
            // '*'
            '*' => Some(Spanned::new(
                Token::Identifier(Identifier::Star),
                (start_offset, 1).into(),
            )),
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
                (start_offset, 1).into(),
            )),
            // '%'
            '%' => Some(Spanned::new(
                Token::Identifier(Identifier::Modulus),
                (start_offset, 1).into(),
            )),
            // '=='
            '=' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::Equals),
                (start_offset, 1).into(),
            )),
            // '='
            '=' => Some(Spanned::new(
                Token::Identifier(Identifier::Assignment),
                (start_offset, 1).into(),
            )),
            // '!='
            '!' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::NotEquals),
                (start_offset, 1).into(),
            )),
            // '!'
            '!' => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalNot),
                (start_offset, 1).into(),
            )),
            // '&&'
            '&' if self.consume_checked('&').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalAnd),
                (start_offset, 1).into(),
            )),
            // '||'
            '|' if self.consume_checked('|').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::LogicalOr),
                (start_offset, 1).into(),
            )),
            // '<='
            '<' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::LessThanOrEqual),
                (start_offset, 1).into(),
            )),
            // '<'
            '<' => Some(Spanned::new(
                Token::Identifier(Identifier::LessThan),
                (start_offset, 1).into(),
            )),
            // '>='
            '>' if self.consume_checked('=').is_some() => Some(Spanned::new(
                Token::Identifier(Identifier::GreaterThanOrEqual),
                (start_offset, 1).into(),
            )),
            // '>'
            '>' => Some(Spanned::new(
                Token::Identifier(Identifier::GreaterThan),
                (start_offset, 1).into(),
            )),
            '\"' => Some(Spanned::new(
                self.parse_string_literal(),
                (start_offset, self.offset - start_offset).into(),
            )),
            c if c.is_numeric() => Some(Spanned::new(
                self.parse_number_literal(current_char),
                (start_offset, self.offset - start_offset).into(),
            )),
            c if c.is_alphabetic() || c == '_' => Some(Spanned::new(
                self.parse_identifier(current_char),
                (start_offset, self.offset - start_offset).into(),
            )),
            _ => None,
        }
    }

    fn parse_string_literal(&mut self) -> Token {
        let mut string = String::new();

        while let Some(c) = self.input.next() {
            self.offset += c.len_utf8();
            if c == '"' {
                break;
            } else if c == '\\' {
                let Some(next) = self.input.next() else {
                    break;
                };

                self.offset += next.len_utf8();

                if next == '\"' {
                    string.push('\"');
                }
            } else {
                string.push(c);
            }
        }
        Token::Literal(Literal::String(string))
    }

    fn parse_number_literal(&mut self, first_char: char) -> Token {
        let mut number = String::new();
        number.push(first_char);

        while let Some(c) = self.input.peek() {
            if c.is_numeric() || c == '.' {
                number.push(c);
                self.offset += c.len_utf8();
                self.input.advance();
            } else {
                break;
            }
        }

        if number.contains('.') {
            Token::Literal(Literal::NumberFloat(number.parse().unwrap()))
        } else {
            Token::Literal(Literal::NumberInt(number.parse().unwrap()))
        }
    }

    fn parse_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);

        while let Some(c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.offset += c.len_utf8();
                self.input.advance();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            // Tokenizer boolean literal
            "true" => Token::Literal(Literal::Bool(true)),
            "false" => Token::Literal(Literal::Bool(false)),
            _ => Token::Identifier(Identifier::from_string(identifier)),
        }
    }
}

impl Tokenizer<'_> {
    fn consume_checked(&mut self, expected: char) -> Option<char> {
        self.input.consume_checked(expected).inspect(|c| {
            self.offset += c.len_utf8();
        })
    }

    fn consume_till(&mut self, expected: &str) -> Option<String> {
        assert!(!expected.is_empty(), "expected must not be empty");
        let mut buffer = String::new();

        let expected = expected.chars().collect::<Vec<_>>();
        let expected = expected.as_slice();

        while let Some(c) = self.input.next() {
            self.offset += c.len_utf8();
            if c == expected[0] {
                let mut found = true;
                for e in &expected[1..] {
                    let next_input = self.input.next()?;
                    self.offset += next_input.len_utf8();

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

impl Iterator for Tokenizer<'_> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub struct TokenizerStream<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> TokenizerStream<'a> {
    pub fn new(input: impl InputStream<Output = char> + 'a) -> Self {
        Self {
            tokenizer: Tokenizer::new(input).peekable(),
        }
    }
}

impl InputStream for TokenizerStream<'_> {
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
