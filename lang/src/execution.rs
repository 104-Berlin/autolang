use source_span::Span;
/// This Module is used to execute a program.
use value::Value;

use crate::{
    error::{Error, ErrorKind, ParseResult},
    module::Module,
    parser::{binary_expression::BinaryExpression, expression::Expr, function::FunctionDecl},
    spanned::Spanned,
    tokenizer::literal::Literal,
};

pub mod value;

pub struct ExecutionContext<'a> {
    pub span: Span,
    pub scopes: Vec<Scope>,
    pub public_functions: Vec<&'a Spanned<FunctionDecl>>,
}

pub struct Scope {
    pub variables: Vec<Spanned<(String, Value)>>,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(module: &'a Spanned<Module>) -> Self {
        Self {
            scopes: vec![Scope {
                variables: Vec::new(),
            }],
            span: module.span,
            public_functions: module.value.functions().iter().collect(),
        }
    }

    pub fn execute(&mut self) -> ParseResult<Value> {
        let Some(main) = self
            .public_functions
            .iter()
            .find(|func| func.value.proto.value.name.value == "main")
        else {
            eprintln!("Error: No main function found");
            return Err(Error::new(self.span, ErrorKind::NoMainFunction));
        };

        self.run_function(main.value.proto.value.name.clone(), vec![])
    }

    fn run_function(
        &mut self,
        func_name: Spanned<String>,
        args: Vec<Spanned<Expr>>,
    ) -> ParseResult<Value> {
        let input_values = args
            .into_iter()
            .map(|arg| self.run_expr(arg))
            .collect::<Vec<_>>();

        let function = self
            .public_functions
            .iter()
            .find(|func| func.value.proto.value.name.value == func_name.value)
            .ok_or(Error::new(
                func_name.span,
                ErrorKind::FunctionNotFound(func_name.value.clone()),
            ))?;

        // Check for provided arguments
        if function.value.proto.value.arguments.value.len() != input_values.len() {
            return Err(Error::new_invalid_number_of_arguments(
                func_name.span,
                function.value.proto.value.arguments.value.len(),
                input_values.len(),
            ));
        }

        let mut scope = Scope {
            variables: Vec::new(),
        };

        for ((arg_name, arg_type), value) in function
            .value
            .proto
            .value
            .arguments
            .value
            .iter()
            .zip(input_values)
        {
            let value = Value {
                value: Box::new(value?),
                type_id: arg_type.value.clone(),
            };

            scope
                .variables
                .push(Spanned::new((arg_name.value.clone(), value), arg_name.span));
        }

        self.scopes.push(scope);

        let res = self.run_expr(function.value.body.clone())?;

        self.scopes.pop();

        Ok(res)
    }

    fn run_expr(&mut self, expr: Spanned<Expr>) -> ParseResult<Value> {
        match expr.value {
            Expr::FunctionCall(name, args) => {
                self.run_function(name.map_span(|_| expr.span), args.value)
            }
            Expr::Variable(name) => {
                let var = self.find_var(&name)?;
                Ok(Spanned::new(var.value.clone(), name.span))
            }
            Expr::Literal(literal) => match literal.value {
                Literal::NumberInt(val) => Ok(Spanned::new(Value::new_int(val), literal.span)),
                Literal::NumberFloat(val) => Ok(Spanned::new(Value::new_float(val), literal.span)),
            },
            Expr::Assignment(var, expr) => {
                let val = self.run_expr(*expr)?;
                let var = self.find_var(&var)?;

                var.value.set_value(&val)?;
                Ok(Spanned::new(val.value, val.span))
            }
            Expr::Let(var_name, type_id, assign) => {
                if let Some(Some(_)) = self.scopes.last().map(|scope| {
                    scope
                        .variables
                        .iter()
                        .find(|var| var.value.0 == var_name.value)
                }) {
                    return Err(Error::new(
                        var_name.span,
                        ErrorKind::VariableAlreadyDeclared(var_name.value.clone()),
                    ));
                }

                let span = assign.span;

                let value = self.run_expr(*assign)?.value;

                if value.type_id != type_id.value {
                    return Err(Error::new_type_mismatch(
                        span,
                        type_id.value.clone(),
                        value.type_id,
                    ));
                }

                self.scopes.last_mut().unwrap().variables.push(Spanned::new(
                    (
                        var_name.value.clone(),
                        Value {
                            value: Box::new(value),
                            type_id: type_id.value.clone(),
                        },
                    ),
                    var_name.span,
                ));

                Ok(Spanned::new(Value::new_void(), span))
            }
            Expr::Binary(Spanned::<BinaryExpression> {
                value: BinaryExpression { lhs, op, rhs },
                ..
            }) => {
                let lhs = self.run_expr(*lhs)?;
                let rhs = self.run_expr(*rhs)?;

                match op.value {
                    crate::parser::binary_expression::BinaryOperator::Add => lhs.value.add(&rhs),
                    crate::parser::binary_expression::BinaryOperator::Substract => {
                        lhs.value.sub(&rhs)
                    }
                    crate::parser::binary_expression::BinaryOperator::Multiply => {
                        lhs.value.mul(&rhs)
                    }
                    crate::parser::binary_expression::BinaryOperator::Divide => lhs.value.div(&rhs),
                }
            }
            Expr::Block(statements, return_expr) => {
                for e in statements {
                    self.run_expr(e)?;
                }
                if let Some(return_expr) = return_expr {
                    self.run_expr(*return_expr)
                } else {
                    Ok(Spanned::new(Value::new_void(), expr.span))
                }
            }
        }
    }
}

// Helpers
impl ExecutionContext<'_> {
    fn find_var(&mut self, name: &Spanned<String>) -> ParseResult<&mut Value> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value) = scope.variables.iter_mut().find_map(
                |Spanned::<(String, Value)> {
                     value: (n, v),
                     span,
                 }| (n == &name.value).then_some(Spanned::new(v, *span)),
            ) {
                return Ok(value);
            }
        }

        Err(Error::new(
            name.span,
            ErrorKind::VariableNotFound(name.value.clone()),
        ))
    }
}