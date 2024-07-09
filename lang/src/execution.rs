use source_span::Span;
/// This Module is used to execute a program.
use value::Value;

use crate::{
    error::{Error, ErrorKind, ParseResult, TypeMismatchReason},
    module::Module,
    parser::{
        binary_expression::{BinaryExpression, BinaryOperator},
        expression::Expr,
        function::FunctionDecl,
        type_def::TypeID,
    },
    spanned::Spanned,
    system_functions::{self, IntoSystem, System},
    tokenizer::literal::Literal,
};

pub mod value;

pub struct ExecutionContext<'a> {
    pub span: Span,
    pub scopes: Vec<Scope>,
    pub public_functions: Vec<&'a Spanned<FunctionDecl>>,
    pub system_functions: Vec<(String, Box<dyn System>)>,
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
            system_functions: Vec::with_capacity(4),
        }
        .register_system_function("print", system_functions::print::print)
        .register_system_function("println", system_functions::print::println)
    }

    pub fn register_system_function<I, S: System + 'static>(
        mut self,
        name: impl Into<String>,
        system: impl IntoSystem<I, System = S>,
    ) -> Self {
        self.system_functions
            .push((name.into(), Box::new(system.into_system())));
        self
    }

    pub fn execute(&mut self) -> ParseResult<Value> {
        let func_name = if let Some(main) = self
            .public_functions
            .iter_mut()
            .find(|func| func.value.proto.value.name.value == "main")
        {
            main.value.proto.value.name.clone()
        } else {
            return Err(Error::new(self.span, ErrorKind::NoMainFunction));
        };

        self.run_function(func_name, &[])
    }

    fn run_function(
        &mut self,
        func_name: Spanned<String>,
        args: &[Spanned<Expr>],
    ) -> ParseResult<Value> {
        // Execute input expressions to the actual values
        let input_values = args
            .iter()
            .map(|arg| self.run_expr(arg))
            .collect::<Vec<_>>();

        // Find the function to call
        let system_function = self
            .system_functions
            .iter()
            .find(|f| f.0 == func_name.value);

        let function = self
            .public_functions
            .iter()
            .find(|func| func.value.proto.value.name.value == func_name.value);

        match (system_function, function) {
            (Some(func), _) => self.run_system_function(func_name, func.1.as_ref(), input_values),
            (None, Some(func)) => self.run_declared_function(func_name.span, func, input_values),
            (None, None) => Err(Error::new(
                func_name.span,
                ErrorKind::FunctionNotFound(func_name.value),
            )),
        }
    }

    fn run_system_function(
        &self,
        call_span: Spanned<String>,
        system: &dyn System,
        arguments: Vec<ParseResult<Value>>,
    ) -> ParseResult<Value> {
        // Check for provided arguments
        /*if proto.arguments.value.len() != arguments.len() {
            return Err(Error::new_invalid_number_of_arguments(
                call_span.span,
                proto.arguments.value.len(),
                arguments.len(),
            ));
        }*/

        let result = system.run(
            arguments
                .into_iter()
                .map(|arg| arg.map(|v| v.value))
                .collect::<Result<Vec<_>, Error>>()?,
        );

        Ok(Spanned::new(result, call_span.span))
    }

    fn run_declared_function(
        &mut self,
        call_span: Span,
        function: &Spanned<FunctionDecl>,
        arguments: Vec<ParseResult<Value>>,
    ) -> ParseResult<Value> {
        // Check for provided arguments
        if function.value.proto.value.arguments.value.len() != arguments.len() {
            return Err(Error::new_invalid_number_of_arguments(
                call_span,
                function.value.proto.value.arguments.value.len(),
                arguments.len(),
            ));
        }

        // Create a new scope for the function
        let mut scope = Scope {
            variables: Vec::new(),
        };

        let return_type = function.value.proto.value.return_type.value.clone();

        // Push input vars to the function stack
        for ((arg_name, arg_type), value) in function
            .value
            .proto
            .value
            .arguments
            .value
            .iter()
            .zip(arguments)
        {
            let value = value?;
            if value.value.type_id != arg_type.value {
                return Err(Error::new_type_mismatch(
                    value.span,
                    arg_type.value.clone(),
                    value.value.type_id.clone(),
                    TypeMismatchReason::FunctionArgument,
                ));
            }

            // Make spanned tuple of the variable name and the value
            // The Span will be the span of the expression which is the input for the function call
            let value = value.map_value(|val| (arg_name.value.clone(), val));

            scope.variables.push(value);
        }

        // Push scope for the body
        self.scopes.push(scope);

        let res = self.run_expr(&function.value.body)?;

        // Pop the scope
        self.scopes.pop();

        if res.value.type_id != return_type {
            // Return types dont match
            return Err(Error::new_type_mismatch(
                res.span,
                return_type,
                res.value.type_id.clone(),
                TypeMismatchReason::FunctionReturn,
            ));
        }

        Ok(res)
    }

    fn run_expr(&mut self, expr: &Spanned<Expr>) -> ParseResult<Value> {
        match &expr.value {
            Expr::FunctionCall(name, args) => {
                self.run_function(name.map_span(|_| expr.span), &args.value)
            }
            Expr::Variable(name) => {
                let var = self.find_var(name)?;
                Ok(Spanned::new(var.value.clone(), name.span))
            }
            Expr::Literal(literal) => match &literal.value {
                Literal::NumberInt(val) => Ok(Spanned::new(Value::new_int(*val), literal.span)),
                Literal::NumberFloat(val) => Ok(Spanned::new(Value::new_float(*val), literal.span)),
                Literal::String(val) => {
                    Ok(Spanned::new(Value::new_string(val.clone()), literal.span))
                }
                Literal::Bool(val) => Ok(Spanned::new(Value::new_bool(*val), literal.span)),
            },
            Expr::Assignment(var, expr) => {
                let val = self.run_expr(expr)?;
                let var = self.find_var(var)?;

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

                let value = self.run_expr(assign)?.value;

                if value.type_id != type_id.value {
                    return Err(Error::new_type_mismatch(
                        span,
                        type_id.value.clone(),
                        value.type_id,
                        TypeMismatchReason::VariableAssignment,
                    ));
                }

                self.scopes
                    .last_mut()
                    .unwrap()
                    .variables
                    .push(Spanned::new((var_name.value.clone(), value), var_name.span));

                Ok(Spanned::new(Value::new_void(), span))
            }
            Expr::Binary(Spanned::<BinaryExpression> {
                value: BinaryExpression { lhs, op, rhs },
                ..
            }) => {
                let lhs = self.run_expr(lhs)?;
                let rhs = self.run_expr(rhs)?;

                match op.value {
                    BinaryOperator::Add => lhs.value.add(&rhs),
                    BinaryOperator::Substract => lhs.value.sub(&rhs),
                    BinaryOperator::Multiply => lhs.value.mul(&rhs),
                    BinaryOperator::Divide => lhs.value.div(&rhs),
                    BinaryOperator::And => lhs.value.and(&rhs),
                    BinaryOperator::Or => lhs.value.or(&rhs),
                    BinaryOperator::Equal => lhs.value.eq(&rhs),
                    BinaryOperator::NotEqual => lhs.value.neq(&rhs),
                    BinaryOperator::LessThan => lhs.value.lt(&rhs),
                    BinaryOperator::LessThanOrEqual => lhs.value.lte(&rhs),
                    BinaryOperator::GreaterThan => lhs.value.gt(&rhs),
                    BinaryOperator::GreaterThanOrEqual => lhs.value.gte(&rhs),
                }
                .map(|v| v.map_span(|_| lhs.span.union(rhs.span)))
            }
            Expr::IfExpression {
                if_block: (condition, then_block),
                else_if_blocks,
                else_block,
            } => {
                let condition = self.run_expr(condition)?;
                let value = condition.value.as_bool().ok_or(Error::new_type_mismatch(
                    condition.span,
                    TypeID::Bool,
                    condition.value.type_id.clone(),
                    TypeMismatchReason::FunctionArgument,
                ))?;

                if value {
                    return self.run_expr(then_block);
                }

                for (else_if_cond, else_if_block) in else_if_blocks {
                    let condition = self.run_expr(else_if_cond)?;
                    let value = condition.value.as_bool().ok_or(Error::new_type_mismatch(
                        condition.span,
                        TypeID::Bool,
                        condition.value.type_id.clone(),
                        TypeMismatchReason::FunctionArgument,
                    ))?;

                    if value {
                        return self.run_expr(else_if_block);
                    }
                }

                if let Some(else_block) = else_block {
                    self.run_expr(else_block)
                } else {
                    Ok(Spanned::new(Value::new_void(), expr.span))
                }
            }
            Expr::Block(statements, return_expr) => {
                for e in statements {
                    self.run_expr(e)?;
                }
                if let Some(return_expr) = return_expr {
                    self.run_expr(return_expr)
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
