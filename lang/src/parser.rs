use binary_expression::{BinaryExpression, BinaryOperator};
use expression::Expr;

use crate::{
    input_stream::InputStream,
    tokenizer::{Identifier, Token, TokenKind, TokenizerStream},
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

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expr {
        let first = self.parse_primary_expression();
        self.parse_binary_expression(first)
    }

    fn parse_primary_expression(&mut self) -> Expr {
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
                    Expr::FunctionCall(name.clone())
                } else {
                    Expr::Variable(name.clone())
                }
            }
            TokenKind::Literal(literal) => {
                self.input.advance();
                Expr::Literal(literal.clone())
            }
            token => panic!("Unexpected token: {:?}", token),
        }
    }

    fn current_precedence(&mut self) -> i16 {
        self.input
            .peek()
            .map(|t| BinaryOperator::try_from(t).map(|op| op.precedence()))
            .unwrap_or(Ok(-1))
            .unwrap_or(-1)
    }

    fn parse_binary_expression(&mut self, mut lhs: Expr) -> Expr {
        while let Some(Ok(op)) = self.input.peek().map(|t| BinaryOperator::try_from(t)) {
            self.input.advance();

            let mut rhs = self.parse_primary_expression();

            if op.precedence() < self.current_precedence() {
                rhs = self.parse_binary_expression(rhs);
            }
            
            lhs = Expr::Binary(BinaryExpression::new(lhs.clone(), op, rhs));
        }
        lhs
    }
}
