use binary_expression::{BinaryExpression, BinaryOperator};
use expression::Expr;
use function::{ArgumentDecl, FunctionDecl, FunctionProto};
use source_span::Span;
use type_def::TypeID;

use crate::{
    error::{ALResult, Error, ErrorKind},
    input_stream::InputStream,
    module::Module,
    spanned::Spanned,
    tokenizer::{identifier::Identifier, token::Token, TokenizerStream},
};

pub mod binary_expression;
pub mod expression;
pub mod function;
pub mod type_def;

/// Creates a parse tree from a stream of tokens.
pub struct Parser {
    input: Box<dyn InputStream<Output = Spanned<Token>>>,
}

impl Parser {
    pub fn new(input: impl InputStream<Output = char> + 'static) -> Self {
        Self {
            input: Box::new(TokenizerStream::new(input)),
        }
    }
}

impl TryInto<Spanned<Expr>> for Parser {
    type Error = Error;

    fn try_into(mut self) -> Result<Spanned<Expr>, Self::Error> {
        self.parse_expression()
    }
}

impl TryInto<Spanned<Module>> for Parser {
    type Error = Error;

    fn try_into(mut self) -> Result<Spanned<Module>, Self::Error> {
        self.parse_module()
    }
}

// -------------------------------------------------------------------------------------------
// Parse Module
impl Parser {
    pub fn parse_module(&mut self) -> ALResult<Module> {
        let mut module = Module::new("main");
        let mut module_span = Span::default();

        while let Ok(Spanned::<Token> { value, span }) = self.peek() {
            module_span.append(span);
            match value {
                Token::Identifier(Identifier::Function) => {
                    self.input.advance();
                    let function = self.parse_function()?;
                    module.add_function(function);
                }
                _ => {
                    return Err(Error::new(
                        span,
                        ErrorKind::UnexpectedToken {
                            found: value,
                            expected: None,
                        },
                    ));
                }
            }
        }

        Ok(Spanned::new(module, module_span))
    }

    fn parse_function(&mut self) -> ALResult<FunctionDecl> {
        let function_name = self.parse_user_defined_identifier()?;
        let proto = self.parse_function_proto(function_name.clone())?;
        let body = self.parse_block_expression()?;

        let span = function_name.span.union(body.span);

        Ok(Spanned::new(FunctionDecl { proto, body }, span))
    }

    fn parse_function_proto(&mut self, name: Spanned<String>) -> ALResult<FunctionProto> {
        let args = self.parse_function_args()?;
        let span = name.span.union(args.span);
        let return_type =
            if let Ok(arrow) = self.consume_checked(Token::Identifier(Identifier::Arrow)) {
                self.parse_type()?.map_span(|span| arrow.span.union(span))
            } else {
                Spanned::new(TypeID::Void, args.span.next())
            };

        Ok(Spanned::new(
            FunctionProto {
                name: name.clone(),
                arguments: args,
                return_type,
            },
            span,
        ))
    }
}

// -------------------------------------------------------------------------------------------
// Parse Expression
impl Parser {
    pub fn parse_expression(&mut self) -> ALResult<Expr> {
        match self.peek()?.value {
            Token::Identifier(Identifier::If) => self.parse_if_expression(),
            Token::Identifier(Identifier::Loop) => self.parse_loop_expression(),
            Token::Identifier(Identifier::Let) => self.parse_let_expression(),
            Token::Identifier(Identifier::LBrace) => self.parse_block_expression(),
            Token::Identifier(Identifier::Return) => self.parse_return_expression(),
            Token::Identifier(Identifier::Break) => {
                self.input.advance();
                Ok(Spanned::new(Expr::Break, self.peek()?.span))
            }
            _ => {
                let lhs = self.parse_primary_expression()?;
                self.parse_binary_expression(lhs, 0)
            }
        }
    }

    fn parse_primary_expression(&mut self) -> ALResult<Expr> {
        let Spanned::<Token> { value, span } = self.peek()?;

        match value {
            Token::Identifier(Identifier::UserDefined(name)) => {
                self.input.advance();
                self.parse_expression_identifier(Spanned::new(name, span))
            }
            Token::Literal(literal) => {
                self.input.advance();
                Ok(Spanned::new(
                    Expr::Literal(Spanned::new(literal, span)),
                    span,
                ))
            }
            Token::Identifier(Identifier::LParen) => {
                self.input.advance();
                let expr = self.parse_expression()?;
                self.consume_checked(Token::Identifier(Identifier::RParen))?;
                Ok(expr)
            }
            _ => Err(Error::new(
                span,
                ErrorKind::UnexpectedToken {
                    found: value,
                    expected: None,
                },
            )),
        }
    }

    /// This parses everything that starts with an identifier. Variables, function calls, etc.
    fn parse_expression_identifier(&mut self, identifier: Spanned<String>) -> ALResult<Expr> {
        if let Ok(Spanned::<Token> {
            value: Token::Identifier(Identifier::LParen),
            span,
        }) = self.peek()
        {
            // Parse function call
            self.input.advance();
            let mut args = Vec::new();
            let mut args_span = span.next();
            loop {
                if let Ok(input) = self.parse_expression() {
                    args_span.append(input.span);
                    args.push(input);
                }

                if let Ok(comma) = self.consume_checked(Token::Identifier(Identifier::Comma)) {
                    args_span.append(comma.span);
                } else {
                    break;
                }
            }
            let r_paren_span = self
                .consume_checked(Token::Identifier(Identifier::RParen))?
                .span;

            let span = identifier.span.union(r_paren_span);

            Ok(Spanned::new(
                Expr::FunctionCall(identifier, Spanned::new(args, args_span)),
                span,
            ))
        } else {
            let span = identifier.span;
            Ok(Spanned::new(Expr::Variable(identifier), span))
        }
    }

    fn current_precedence(&mut self) -> i16 {
        self.peek()
            .map(|t| BinaryOperator::try_from(t).map(|op| op.precedence()))
            .unwrap_or(Ok(-1))
            .unwrap_or(-1)
    }

    fn parse_binary_expression(
        &mut self,
        mut lhs: Spanned<Expr>,
        precendence: i16,
    ) -> ALResult<Expr> {
        while let Ok(token) = self.peek() {
            let op = match Spanned::<BinaryOperator>::try_from(token) {
                Ok(op) => op,
                Err(_) => {
                    return Ok(lhs);
                }
            };
            if op.value.precedence() < precendence {
                return Ok(lhs);
            }

            self.input.advance();

            let mut rhs = self.parse_primary_expression()?;

            if op.value.precedence() < self.current_precedence() {
                rhs = self.parse_binary_expression(rhs, op.value.precedence() + 1)?;
            }

            let span = lhs.span.union(rhs.span);

            lhs = Spanned::new(
                Expr::Binary(Spanned::new(BinaryExpression::new(lhs, op, rhs), span)),
                span,
            );
        }
        Ok(lhs)
    }

    fn parse_block_expression(&mut self) -> ALResult<Expr> {
        let mut block = Vec::new();

        let span = self
            .consume_checked(Token::Identifier(Identifier::LBrace))?
            .span;

        let mut return_expression = None;

        while !self.is_next_token(Token::Identifier(Identifier::RBrace)) {
            let expr = self.parse_expression()?;

            // We expect a semicolon after each expression in a block, or we are at the end of the block.
            match self.consume_checked(Token::Identifier(Identifier::Semicolon)) {
                Ok(_) => {
                    block.push(expr);
                }
                Err(_) if self.is_next_token(Token::Identifier(Identifier::RBrace)) => {
                    return_expression = Some(Box::new(expr));
                    break;
                }
                // If expressions dont need a semicolon
                Err(_)
                    if matches!(expr.value, Expr::IfExpression { .. })
                        || matches!(expr.value, Expr::Loop(_)) =>
                {
                    block.push(expr);
                }
                Err(e) => return Err(e),
            }
        }

        let span = span.union(
            self.consume_checked(Token::Identifier(Identifier::RBrace))?
                .span,
        );

        Ok(Spanned::new(Expr::Block(block, return_expression), span))
    }

    fn parse_return_expression(&mut self) -> ALResult<Expr> {
        let span = self
            .consume_checked(Token::Identifier(Identifier::Return))?
            .span;

        if self.is_next_token(Token::Identifier(Identifier::Semicolon)) {
            self.input.advance();
            return Ok(Spanned::new(Expr::Return(None), span));
        }

        let expr = self.parse_expression()?;
        let span = span.union(expr.span);

        Ok(Spanned::new(Expr::Return(Some(Box::new(expr))), span))
    }

    fn parse_let_expression(&mut self) -> ALResult<Expr> {
        let span_start = self
            .consume_checked(Token::Identifier(Identifier::Let))?
            .span;
        let var_name = self.parse_user_defined_identifier()?;

        self.consume_checked(Token::Identifier(Identifier::Colon))?;
        let type_id = self.parse_type()?;

        self.consume_checked(Token::Identifier(Identifier::Assignment))?;
        let assign_to = self.parse_expression()?;

        let span = span_start.union(assign_to.span);
        Ok(Spanned::new(
            Expr::Let(var_name.clone(), type_id, Box::new(assign_to)),
            span,
        ))
    }

    fn parse_if_expression(&mut self) -> ALResult<Expr> {
        self.consume_checked(Token::Identifier(Identifier::If))?;

        let condition = Box::new(self.parse_expression()?);
        let then_block = Box::new(self.parse_block_expression()?);

        let mut else_if_blocks = Vec::new();

        let mut else_block = None;

        while self
            .consume_checked(Token::Identifier(Identifier::Else))
            .is_ok()
        {
            match self.consume_checked(Token::Identifier(Identifier::If)) {
                Ok(_) => else_if_blocks.push((
                    Box::new(self.parse_expression()?),
                    Box::new(self.parse_block_expression()?),
                )),
                Err(_) => {
                    else_block = Some(Box::new(self.parse_block_expression()?));
                    break;
                }
            }
        }

        let span = condition
            .span
            .union(else_block.as_ref().unwrap_or(&then_block).span);

        Ok(Spanned::new(
            Expr::IfExpression {
                if_block: (condition, then_block),
                else_if_blocks,
                else_block,
            },
            span,
        ))
    }

    fn parse_loop_expression(&mut self) -> ALResult<Expr> {
        let loop_span = self
            .consume_checked(Token::Identifier(Identifier::Loop))?
            .span;
        let expr = Box::new(self.parse_block_expression()?);

        let span = loop_span.union(expr.span);
        Ok(Spanned::new(Expr::Loop(expr), span))
    }
}

// -------------------------------------------------------------------------------------------
// Simple Parsers

impl Parser {
    fn parse_user_defined_identifier(&mut self) -> ALResult<String> {
        match self.peek()? {
            Spanned::<Token> {
                value: Token::Identifier(Identifier::UserDefined(name)),
                span,
            } => {
                self.input.advance();
                Ok(Spanned::new(name, span))
            }
            tok => Err(Error::new_unexpected_token(tok, None)),
        }
    }

    /// Parses a list of function arguments.
    ///
    /// This is a list of `name: type` pairs separated by commas.
    ///
    /// The list is enclosed in parentheses.
    fn parse_function_args(&mut self) -> ALResult<Vec<ArgumentDecl>> {
        let mut args = Vec::new();

        let l_paren_span = self
            .consume_checked(Token::Identifier(Identifier::LParen))?
            .span;

        if let Ok(Spanned::<Token> { span, .. }) =
            self.consume_checked(Token::Identifier(Identifier::RParen))
        {
            // No Params
            return Ok(Spanned::new(vec![], l_paren_span.union(span)));
        }

        loop {
            let name = self.parse_user_defined_identifier()?;
            self.consume_checked(Token::Identifier(Identifier::Colon))?;
            let ty = self.parse_type()?;
            args.push((name, ty));

            // No more comma. Next token must be RParen
            if self
                .consume_checked(Token::Identifier(Identifier::Comma))
                .is_err()
            {
                break;
            }
        }

        let r_paren_span = self
            .consume_checked(Token::Identifier(Identifier::RParen))?
            .span;

        Ok(Spanned::new(args, l_paren_span.union(r_paren_span)))
    }

    fn parse_type(&mut self) -> ALResult<TypeID> {
        match self.peek()? {
            Spanned::<Token> {
                value: Token::Identifier(Identifier::UserDefined(type_name)),
                span,
            } => {
                self.input.advance();
                match type_name.as_str() {
                    "int" => Ok(Spanned::new(TypeID::Int, span)),
                    "float" => Ok(Spanned::new(TypeID::Float, span)),
                    "String" => Ok(Spanned::new(TypeID::String, span)),
                    _ => Ok(Spanned::new(TypeID::User(type_name), span)),
                }
            }
            tok => Err(Error::new_unexpected_token(tok, None)),
        }
    }
}
// Parser helpers
impl Parser {
    fn is_next_token(&mut self, expected: Token) -> bool {
        self.peek().map_or(false, |t| t.value == expected)
    }

    #[allow(dead_code)]
    fn expect_token(&mut self, expected: Token) -> ALResult<Token> {
        let token = self.peek()?;

        if token.value == expected {
            Ok(token)
        } else {
            Err(Error::new_unexpected_token(token, Some(expected)))
        }
    }

    fn consume_checked(&mut self, expected: Token) -> ALResult<Token> {
        let token = self.peek()?;

        if token.value == expected {
            self.input.advance();
            Ok(token)
        } else {
            Err(Error::new_unexpected_token(token, Some(expected)))
        }
    }

    fn peek(&mut self) -> ALResult<Token> {
        self.input
            .peek()
            .ok_or(Error::new(Span::default(), ErrorKind::UnexpectedEOF))
    }
}
