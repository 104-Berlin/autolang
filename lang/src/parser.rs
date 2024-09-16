use binary_expression::{BinaryExpression, BinaryOperator};
use expression::{DotExpr, Expr};
use function::{ArgumentDecl, FunctionDecl, FunctionProto};
use miette::{Context, Error, SourceOffset, SourceSpan};
use reset_iterator::ResetIterator;
use structs::Struct;
use type_def::TypeID;

use crate::{
    error::UnexpectedToken,
    input_stream::InputStream,
    module::Module,
    spanned::{SpanExt, Spanned, WithSpan},
    tokenizer::{identifier::Identifier, token::Token, Tokenizer},
    ALResult,
};

pub mod binary_expression;
pub mod expression;
pub mod function;
pub mod structs;
pub mod type_def;

/// A parse tree from a stream of tokens.
/// # Example
/// ```rust
/// use lang::prelude::*;
///
/// let input = "let x: i32 = 42;";
/// let mut parser = Parser::new(input);
/// let expr = parser.parse_expression().unwrap();
/// // Or
/// let mut parser = Parser::new(input);
/// let expr: Spanned<Expr> = parser.try_into().unwrap();
/// ```
pub struct Parser<'a> {
    input: ResetIterator<Tokenizer<'a>>,
    last_offset: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from an input stream.
    /// # Arguments
    /// * `input` - The input stream to parse.
    /// # Returns
    /// A new parser.
    pub fn new(input: impl InputStream<Output = char> + 'a) -> Self {
        Self {
            input: Tokenizer::new(input).into(),
            last_offset: 0,
        }
    }
}

impl TryInto<Spanned<Expr>> for Parser<'_> {
    type Error = Error;

    fn try_into(mut self) -> Result<Spanned<Expr>, Self::Error> {
        self.parse_expression()
    }
}

impl TryInto<Spanned<Module>> for Parser<'_> {
    type Error = Error;

    fn try_into(mut self) -> Result<Spanned<Module>, Self::Error> {
        self.parse_module()
    }
}

// -------------------------------------------------------------------------------------------
// Parse Module
impl Parser<'_> {
    pub fn parse_module(&mut self) -> ALResult<Module> {
        let mut module = Module::new("main");
        let mut module_span = SourceSpan::new(SourceOffset::from(0), 0);

        while let Ok(Spanned::<Token> { value, span }) = self.peek() {
            module_span = module_span.union(&span);
            match value {
                Token::Identifier(Identifier::Function) => {
                    self.consume();
                    let function = self.parse_function()?;
                    module.add_function(function);
                }
                Token::Identifier(Identifier::Struct) => {
                    self.consume();
                    let struct_name = self.parse_user_defined_identifier()?;
                    let struct_decl = self.parse_struct()?;
                    module.add_struct(struct_name, struct_decl);
                }
                _ => {
                    return Err(UnexpectedToken {
                        found: value,
                        span,
                        expected: "Expected function or struct".into(),
                    })
                    .wrap_err("Parsing module");
                }
            }
        }

        Ok(Spanned::new(module, module_span))
    }

    fn parse_function(&mut self) -> ALResult<FunctionDecl> {
        let function_name = self.parse_user_defined_identifier()?;
        let proto = self.parse_function_proto(function_name.clone())?;
        let body = self.parse_block_expression()?;

        let span = function_name.span.union(&body.span);

        Ok(Spanned::new(FunctionDecl { proto, body }, span))
    }

    fn parse_function_proto(&mut self, name: Spanned<String>) -> ALResult<FunctionProto> {
        let args = self.parse_function_args_decl()?;
        let span = name.span.union(&args.span);
        let return_type =
            if let Ok(arrow) = self.consume_checked(Token::Identifier(Identifier::Arrow)) {
                self.parse_type()?.map_span(|span| arrow.span.union(&span))
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

    fn parse_struct(&mut self) -> ALResult<Struct> {
        let fields = self.parse_struct_fields()?;

        Ok(Spanned::new(Struct::new(fields.value), fields.span))
    }

    fn parse_struct_fields(&mut self) -> ALResult<Vec<Spanned<(String, TypeID)>>> {
        let start_span = self
            .consume_checked(Token::Identifier(Identifier::LBrace))?
            .span;

        let mut fields = Vec::new();
        loop {
            fields.push(self.parse_struct_field()?);

            if let Ok(rbrace) = self.consume_checked(Token::Identifier(Identifier::RBrace)) {
                let span = start_span.union(&rbrace.span);

                return Ok(Spanned::new(fields, span));
            }
        }
    }

    fn parse_struct_field(&mut self) -> ALResult<(String, TypeID)> {
        let name = self.parse_user_defined_identifier()?;
        self.consume_checked(Token::Identifier(Identifier::Colon))?;
        let ty = self.parse_type()?;
        self.consume_checked(Token::Identifier(Identifier::Semicolon))?;

        Ok(Spanned::new(
            (name.value, ty.value),
            name.span.union(&ty.span),
        ))
    }
}

// -------------------------------------------------------------------------------------------
// Parse Expression
impl Parser<'_> {
    pub fn parse_expression(&mut self) -> ALResult<Expr> {
        match self.peek()?.value {
            Token::Identifier(Identifier::If) => self.parse_if_expression(),
            Token::Identifier(Identifier::Loop) => self.parse_loop_expression(),
            Token::Identifier(Identifier::Let) => self.parse_let_expression(),
            Token::Identifier(Identifier::LBrace) => self.parse_block_expression(),
            Token::Identifier(Identifier::Return) => self.parse_return_expression(),
            Token::Identifier(Identifier::Break) => {
                let span = self.peek()?.span;
                self.consume();
                Ok(Spanned::new(Expr::Break, span))
            }
            _ => {
                let lhs = self.parse_primary_expression()?;
                self.parse_binary_expression(lhs, 0)
            }
        }
    }

    fn parse_primary_expression(&mut self) -> ALResult<Expr> {
        let Spanned::<Token> { value, span } = self.peek()?;

        let mut lhs = match value {
            Token::Identifier(Identifier::UserDefined(_)) => self.parse_expression_identifier(),
            Token::Literal(literal) => {
                self.consume();
                Ok(Spanned::new(
                    Expr::Literal(Spanned::new(literal, span)),
                    span,
                ))
            }
            Token::Identifier(Identifier::LParen) => {
                self.consume();
                let expr = self.parse_expression()?;
                self.consume_checked(Token::Identifier(Identifier::RParen))?;
                Ok(expr)
            }
            _ => Err(UnexpectedToken {
                found: value,
                span: self.last_offset.into(),
                expected: "Expected expression".into(),
            }
            .into()),
        }?;

        // Check if we have a dot operator
        while self
            .consume_checked(Token::Identifier(Identifier::Dot))
            .is_ok()
        {
            let rhs = self.parse_expression_function_call_or_variable()?;
            let span = span.union(&rhs.span);

            lhs = Spanned::new(
                Expr::Dot {
                    lhs: Box::new(lhs),
                    rhs,
                },
                span,
            );
        }

        Ok(lhs)
    }

    /// This parses everything that starts with an identifier. Variables, function calls, etc.
    fn parse_expression_identifier(&mut self) -> ALResult<Expr> {
        self.try_parse(Self::parse_struct_literal)
            .wrap_err("Parsing struct literal")
            .or_else(|_| {
                self.parse_expression_function_call_or_variable()
                    .map(|e| e.map_value(Into::into))
            })
    }

    fn parse_struct_literal(&mut self) -> ALResult<Expr> {
        let identifier = self.parse_user_defined_identifier()?;

        self.consume_checked(Token::Identifier(Identifier::LBrace))?;

        let mut fields = Vec::new();
        while !self.is_next_token(Token::Identifier(Identifier::RBrace)) {
            let name = self.parse_user_defined_identifier()?;
            self.consume_checked(Token::Identifier(Identifier::Colon))?;
            let expr = self.parse_expression()?;

            fields.push((name, expr));

            if !self.is_next_token(Token::Identifier(Identifier::RBrace)) {
                // No RBrace? Next token must be a comma
                self.consume_checked(Token::Identifier(Identifier::Comma))?;
            }
        }
        let r_brace_span = self
            .consume_checked(Token::Identifier(Identifier::RBrace))?
            .span;

        let span = identifier.span.union(&r_brace_span);

        Ok(Spanned::new(Expr::StructLiteral(identifier, fields), span))
    }

    fn parse_expression_function_call_or_variable(&mut self) -> ALResult<DotExpr> {
        let identifier = self.parse_user_defined_identifier()?;

        match self.consume_checked(Token::Identifier(Identifier::LParen)) {
            Ok(_) => {
                let mut args = Vec::new();
                loop {
                    if let Ok(input) = self.parse_expression() {
                        args.push(input);
                    }

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

                let span = identifier.span.union(&r_paren_span);

                Ok(Spanned::new(DotExpr::FunctionCall(identifier, args), span))
            }
            _ => {
                let span = identifier.span;
                Ok(Spanned::new(DotExpr::Variable(identifier), span))
            }
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

            self.consume();

            let mut rhs = self.parse_primary_expression()?;

            if op.value.precedence() < self.current_precedence() {
                rhs = self.parse_binary_expression(rhs, op.value.precedence() + 1)?;
            }

            let span = lhs.span.union(&rhs.span);

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
            &self
                .consume_checked(Token::Identifier(Identifier::RBrace))?
                .span,
        );

        Ok(Spanned::new(Expr::Block(block, return_expression), span))
    }

    fn parse_return_expression(&mut self) -> ALResult<Expr> {
        let span = self
            .consume_checked(Token::Identifier(Identifier::Return))?
            .span;

        if self.is_next_token(Token::Identifier(Identifier::Semicolon)) {
            self.consume();
            return Ok(Spanned::new(Expr::Return(None), span));
        }

        let expr = self.parse_expression()?;
        let span = span.union(&expr.span);

        Ok(Spanned::new(Expr::Return(Some(Box::new(expr))), span))
    }

    fn parse_let_expression(&mut self) -> ALResult<Expr> {
        let span_start = self
            .consume_checked(Token::Identifier(Identifier::Let))?
            .span;
        let var_name = self.parse_user_defined_identifier()?;

        let type_id = if self
            .consume_checked(Token::Identifier(Identifier::Colon))
            .is_ok()
        {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume_checked(Token::Identifier(Identifier::Assignment))?;
        let assign_to = self.parse_expression()?;

        let span = span_start.union(&assign_to.span);
        Ok(Spanned::new(
            Expr::Let(var_name.clone(), type_id, Box::new(assign_to)),
            span,
        ))
    }

    fn parse_if_expression(&mut self) -> ALResult<Expr> {
        self.consume_checked(Token::Identifier(Identifier::If))?;

        let if_condition = Box::new(self.parse_expression().wrap_err("Parsing if condition")?);
        let if_block = Box::new(self.parse_block_expression().wrap_err("Parse if block")?);

        let mut else_block = None;

        if self
            .consume_checked(Token::Identifier(Identifier::Else))
            .is_ok()
        {
            else_block = Some(Box::new(
                self.parse_expression().wrap_err("Parsing else block")?,
            ));
        }

        let span = if_condition
            .span
            .union(&else_block.as_ref().unwrap_or(&if_block).span);

        Ok(Spanned::new(
            Expr::IfExpression {
                if_block: (if_condition, if_block),
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

        let span = loop_span.union(&expr.span);
        Ok(Spanned::new(Expr::Loop(expr), span))
    }
}

// -------------------------------------------------------------------------------------------
// Simple Parsers

impl Parser<'_> {
    fn parse_user_defined_identifier(&mut self) -> ALResult<String> {
        match self.peek()? {
            Spanned::<Token> {
                value: Token::Identifier(Identifier::UserDefined(name)),
                span,
            } => {
                self.consume();
                Ok(Spanned::new(name, span))
            }
            tok => Err(UnexpectedToken {
                found: tok.value,
                span: tok.span,
                expected: "Expected user defined identifier".into(),
            }
            .into()),
        }
    }

    /// Parses a list of function arguments.
    ///
    /// This is a list of `name: type` pairs separated by commas.
    ///
    /// The list is enclosed in parentheses.
    fn parse_function_args_decl(&mut self) -> ALResult<Vec<ArgumentDecl>> {
        let mut args = Vec::new();

        let l_paren_span = self
            .consume_checked(Token::Identifier(Identifier::LParen))?
            .span;

        if let Ok(Spanned::<Token> { span, .. }) =
            self.consume_checked(Token::Identifier(Identifier::RParen))
        {
            // No Params
            return Ok(Spanned::new(vec![], l_paren_span.union(&span)));
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

        Ok(Spanned::new(args, l_paren_span.union(&r_paren_span)))
    }

    fn parse_type(&mut self) -> ALResult<TypeID> {
        let token = self.peek()?;
        match token.value {
            Token::Identifier(Identifier::UserDefined(type_name)) => {
                self.consume();
                Ok(Spanned::new(TypeID::from_string(&type_name), token.span))
            }
            Token::Identifier(Identifier::LParen) => self.parse_function_type(),
            _ => Err(UnexpectedToken {
                found: token.value,
                span: token.span,
                expected: "Expected a type".into(),
            }
            .into()),
        }
    }

    /// ```(type, type, ...) -> type```
    ///
    /// ```(type, type, ...)``` - Return Void
    fn parse_function_type(&mut self) -> ALResult<TypeID> {
        let token = self.consume_checked(Token::Identifier(Identifier::LParen))?;

        let mut span = token.span;

        let mut args = Vec::new();

        while !self.is_next_token(Token::Identifier(Identifier::RParen)) {
            let typ = self.parse_type()?;
            span = span.union(&typ.span);
            args.push(typ.value);

            if self
                .consume_checked(Token::Identifier(Identifier::Comma))
                .is_err()
            {
                span = span.union(
                    &self
                        .consume_checked(Token::Identifier(Identifier::RParen))?
                        .span,
                );
            }
        }
        // If we have an arrow, we have a return type
        let return_type = match self.consume_checked(Token::Identifier(Identifier::Arrow)) {
            Ok(_) => {
                let typ = self.parse_type()?;
                span = span.union(&typ.span);
                typ.value
            }
            Err(_) => TypeID::Void,
        };

        Ok(TypeID::Function(args, Box::new(return_type)).with_span(span))
    }
}
// Parser helpers
impl Parser<'_> {
    fn consume(&mut self) -> Option<&Spanned<Token>> {
        self.input
            .consume()
            .inspect(|t| self.last_offset = t.span.offset() + t.span.len())
    }

    #[allow(dead_code)]
    fn try_parse<T, F>(&mut self, f: F) -> ALResult<T>
    where
        F: FnOnce(&mut Self) -> ALResult<T>,
    {
        self.input.push_end();
        let last_offset_cache = self.last_offset;
        let result = f(self);
        if result.is_err() {
            self.input.reset();
            self.last_offset = last_offset_cache;
        } else {
            self.input.pop_end();
        }
        result
    }

    fn is_next_token(&mut self, expected: Token) -> bool {
        self.peek().map_or(false, |t| t.value == expected)
    }

    fn consume_checked(&mut self, expected: Token) -> ALResult<Token> {
        let token = self.peek()?;

        if token.value == expected {
            self.consume();
            Ok(token)
        } else {
            Err(Error::from(UnexpectedToken {
                found: token.value,
                span: SourceSpan::from(self.last_offset),
                expected: expected.into(),
            }))
        }
    }

    /// Peeks the next token in the input stream.
    /// # Errors
    /// Returns ErrorKind::UnexpectedEOF if the input stream is empty.
    fn peek(&mut self) -> ALResult<Token> {
        self.input
            .peek()
            .cloned()
            .ok_or(miette::Error::msg("Unexpected EOF"))
    }
}
