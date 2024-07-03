use binary_expression::{BinaryExpression, BinaryOperator};
use expression::Expr;

use crate::{
    error::{Error, ErrorKind, ParseResult}, input_stream::InputStream, tokenizer::{Identifier, Token, TokenKind, TokenizerStream}
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
        self.parse_binary_expression(first)
    }

    fn parse_primary_expression(&mut self) -> ParseResult<Expr> {
        let token = self.input.peek().unwrap();
        match token.kind() {
            TokenKind::Identifier(Identifier::UserDefined(name)) => {
                self.input.advance();
                if let Some(TokenKind::Identifier(Identifier::LParen)) =
                    self.input.peek().map(|t| t.kind().clone())
                {
                    self.input.advance();
                    let mut args = Vec::new();
                    while let TokenKind::Identifier(Identifier::RParen) =
                        self.input.peek().unwrap().kind()
                    {
                        args.push(self.parse_expression());
                        if let TokenKind::Identifier(Identifier::Comma) =
                            self.input.peek().unwrap().kind()
                        {
                            self.input.advance();
                        }
                    }
                    Ok(Expr::FunctionCall(name.clone()))
                } else {
                    Ok(Expr::Variable(name.clone()))
                }
            }
            TokenKind::Literal(literal) => {
                self.input.advance();
                Ok(Expr::Literal(literal.clone()))
            }
            _ => Err(Error::new(token.span(), ErrorKind::UnexpectedToken)),
        }
    }

    fn current_precedence(&mut self) -> i16 {
        self.input
            .peek()
            .map(|t| BinaryOperator::try_from(t).map(|op| op.precedence()))
            .unwrap_or(Ok(-1))
            .unwrap_or(-1)
    }

    fn parse_binary_expression(&mut self, mut lhs: Expr) -> ParseResult<Expr> {
        while let Some(op) = self.input.peek().map(|t| BinaryOperator::try_from(t)) {
            let op = op?;
            self.input.advance();

            let mut rhs = self.parse_primary_expression()?;

            if op.precedence() < self.current_precedence() {
                rhs = self.parse_binary_expression(rhs)?;
            }
            
            lhs = Expr::Binary(BinaryExpression::new(lhs.clone(), op, rhs));
        }
        Ok(lhs)
    }
}
