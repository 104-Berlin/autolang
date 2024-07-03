use binary_expression::{BinaryExpression, BinaryOperator};
use expression::Expr;
use source_span::Span;

use crate::{
    error::{Error, ErrorKind, ParseResult},
    input_stream::InputStream,
    tokenizer::{
        identifier::Identifier,
        token::{Token, TokenKind},
        TokenizerStream,
    },
};

pub mod binary_expression;
pub mod expression;
pub mod function;
pub mod type_def;

/// Creates a parse tree from a stream of tokens.
pub struct Parser {
    input: Box<dyn InputStream<Output = Token>>,
}

impl Parser {
    pub fn new(input: impl InputStream<Output = char> + 'static) -> Self {
        Self {
            input: Box::new(TokenizerStream::new(input)),
        }
    }

    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        let first = self.parse_primary_expression()?;
        self.parse_binary_expression(first, 0)
    }

    fn parse_primary_expression(&mut self) -> ParseResult<Expr> {
        let Token { span, kind } = self.peek()?;

        match kind {
            TokenKind::Identifier(Identifier::UserDefined(name)) => {
                self.input.advance();
                self.parse_expression_identifier(&name)
            }
            TokenKind::Literal(literal) => {
                self.input.advance();
                Ok(Expr::Literal(literal.clone()))
            }
            TokenKind::Identifier(Identifier::LParen) => {
                self.input.advance();
                let expr = self.parse_expression()?;
                self.consume_checked(TokenKind::Identifier(Identifier::RParen))?;
                Ok(expr)
            }
            _ => Err(Error::new(
                span,
                ErrorKind::UnexpectedToken {
                    found: kind,
                    expected: None,
                },
            )),
        }
    }

    /// This parses everything that starts with an identifier. Variables, function calls, etc.
    fn parse_expression_identifier(&mut self, identifier: &str) -> ParseResult<Expr> {
        if let Ok(TokenKind::Identifier(Identifier::LParen)) = self.peek().map(|t| t.kind) {
            self.input.advance();
            let mut args = Vec::new();
            loop {
                if let Ok(input) = self.parse_expression() {
                    args.push(input);
                }

                if self.is_next_token(|t| matches!(t, TokenKind::Identifier(Identifier::Comma))) {
                    self.input.advance();
                } else {
                    break;
                }
            }
            self.expect_token(TokenKind::Identifier(Identifier::RParen))?;
            self.input.advance();
            Ok(Expr::FunctionCall(identifier.to_string()))
        } else {
            Ok(Expr::Variable(identifier.to_string()))
        }
    }

    fn current_precedence(&mut self) -> i16 {
        self.peek()
            .map(|t| BinaryOperator::try_from(t).map(|op| op.precedence()))
            .unwrap_or(Ok(-1))
            .unwrap_or(-1)
    }

    fn parse_binary_expression(&mut self, mut lhs: Expr, precendence: i16) -> ParseResult<Expr> {
        while let Ok(token) = self.peek() {
            let op = match BinaryOperator::try_from(token) {
                Ok(op) => op,
                Err(_) => {
                    return Ok(lhs);
                }
            };
            if op.precedence() < precendence {
                return Ok(lhs);
            }

            self.input.advance();

            let mut rhs = self.parse_primary_expression()?;

            if op.precedence() < self.current_precedence() {
                rhs = self.parse_binary_expression(rhs, op.precedence() + 1)?;
            }

            lhs = Expr::Binary(BinaryExpression::new(lhs.clone(), op, rhs));
        }
        Ok(lhs)
    }

    fn is_next_token<F>(&mut self, p: F) -> bool
    where
        F: FnOnce(TokenKind) -> bool,
    {
        self.peek().map_or(false, |t| p(t.kind))
    }

    fn expect_token(&mut self, expected: TokenKind) -> ParseResult<()> {
        let Token { kind, span } = self.peek()?;

        if kind == expected {
            Ok(())
        } else {
            Err(Error::unexpected_token(span, kind, Some(expected)))
        }
    }

    fn consume_checked(&mut self, expected: TokenKind) -> ParseResult<()> {
        let Token { kind, span } = self.peek()?;

        if kind == expected {
            self.input.advance();
            Ok(())
        } else {
            Err(Error::unexpected_token(span, kind, Some(expected)))
        }
    }

    fn peek(&mut self) -> ParseResult<Token> {
        self.input
            .peek()
            .ok_or(Error::new(Span::default(), ErrorKind::UnexpectedEOF))
    }
}
