use std::collections::HashMap;

use source_span::Span;
/// This Module is used to execute a program.
use value::Value;

use crate::{
    error::{ALResult, Error, ErrorKind, TypeMismatchReason},
    module::Module,
    parser::{
        binary_expression::{BinaryExpression, BinaryOperator},
        expression::{DotExpr, Expr},
        function::FunctionDecl,
        structs::StructValue,
        type_def::{TypeDef, TypeID},
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
    pub public_types: HashMap<String, Spanned<TypeDef>>,
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
            public_types: module
                .value
                .structs()
                .iter()
                .map(|s| (s.0.value.clone(), s.1.clone().map_value(TypeDef::Struct)))
                .collect(),
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

    pub fn execute(&mut self) -> ALResult<Value> {
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
    ) -> ALResult<Value> {
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
        arguments: Vec<ALResult<Value>>,
    ) -> ALResult<Value> {
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
        arguments: Vec<ALResult<Value>>,
    ) -> ALResult<Value> {
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

        let res = self.run_expr(&function.value.body).or_else(|err| {
            let (kind, span) = err.split();
            match kind {
                ErrorKind::Return(val) => Ok(Spanned::new(val, span)),
                _ => Err(Error::new(span, kind)),
            }
        })?;

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

    fn run_expr(&mut self, expr: &Spanned<Expr>) -> ALResult<Value> {
        match &expr.value {
            Expr::Dot { lhs, rhs } => {
                let span = lhs.span;
                let lhs = self.run_expr(lhs)?;
                match &rhs.value {
                    DotExpr::Variable(name) => {
                        let type_def =
                            self.find_type_def(&lhs.clone().map_value(|value| value.type_id))?;
                        match type_def.value {
                            TypeDef::Struct(strct) => {
                                strct.fields.iter().position(|f| f.value.0 == name.value).map(
                                    |index| lhs
                                                    .value
                                                    .as_struct()
                                                    .expect("Value ist not a struct. Can't happen, because we check if type is struct")
                                                    .get_field(index)
                                                    .expect("Field must exist. Or we try to access wrong struct")
                                                    .clone()
                                ).ok_or(Error::new(expr.span, ErrorKind::StructFieldNotFound(name.value.clone())))
                            }
                            _ => Err(Error::new(span.next().union(name.span), ErrorKind::FailedToAccessField(type_def.value))),
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Expr::FunctionCall(name, args) => self.run_function(name.map_span(|_| expr.span), args),
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
            Expr::StructLiteral(name, field_inits) => {
                let Spanned::<TypeDef> { value, .. } =
                    self.find_type_def(&name.clone().map_value(TypeID::User))?;

                let TypeDef::Struct(struct_def) = value else {
                    return Err(Error::new(
                        name.span,
                        ErrorKind::TypeNotFound(name.value.clone()),
                    ));
                };

                let mut struct_value = StructValue::default();
                for struct_def_field in struct_def.fields.iter() {
                    let field = field_inits
                        .iter()
                        .find(|f| f.0.value == struct_def_field.value.0)
                        .map(|f| self.run_expr(&f.1))
                        .ok_or(Error::new(
                            expr.span,
                            ErrorKind::StructFieldNotInitialized(struct_def_field.value.0.clone()),
                        ))??;

                    // Handle invalid type
                    if field.value.type_id != struct_def_field.value.1 {
                        return Err(Error::new_type_mismatch(
                            field.span,
                            struct_def_field.value.1.clone(),
                            field.value.type_id,
                            TypeMismatchReason::VariableAssignment,
                        ));
                    }
                    struct_value.push_field(field);
                }
                // Check if we try to initialize a field that is not in the struct
                for field in field_inits {
                    if !struct_def.fields.iter().any(|f| f.value.0 == field.0.value) {
                        return Err(Error::new(
                            field.0.span,
                            ErrorKind::StructFieldNotFound(field.0.value.clone()),
                        ));
                    }
                }

                Ok(Spanned::new(
                    Value::new_struct(name.value.clone(), struct_value),
                    expr.span,
                ))
            }
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

                if let Some(type_id) = type_id {
                    if value.type_id != type_id.value {
                        return Err(Error::new_type_mismatch(
                            span,
                            type_id.value.clone(),
                            value.type_id,
                            TypeMismatchReason::VariableAssignment,
                        ));
                    }
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
                if matches!(op.value, BinaryOperator::Assign) {
                    if let Expr::Variable(lhs_var) = &lhs.value {
                        let rhs = self.run_expr(rhs)?;
                        let var = self.find_var(lhs_var)?;

                        var.value.set_value(&rhs)?;
                        return Ok(Spanned::new(rhs.value, expr.span));
                    } else {
                        return Err(Error::new(lhs.span, ErrorKind::InvalidAssignmentTarget));
                    }
                }

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
                    // Assign already covered
                    _ => unreachable!(),
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

            Expr::Loop(expr) => loop {
                match self.run_expr(expr) {
                    Ok(_) => {}
                    Err(err) => {
                        let (kind, span) = err.split();
                        match kind {
                            ErrorKind::Break => break Ok(Spanned::new(Value::new_void(), span)),
                            ErrorKind::Continue => continue,
                            _ => return Err(Error::new(span, kind)),
                        }
                    }
                }
            },

            Expr::Return(ret_val) => {
                let value = ret_val
                    .as_ref()
                    .map(|e| self.run_expr(e))
                    .transpose()?
                    .unwrap_or(Spanned::new(Value::new_void(), expr.span));
                Err(Error::new(value.span, ErrorKind::Return(value.value)))
            }
            Expr::Break => Err(Error::new(expr.span, ErrorKind::Break)),
            Expr::Continue => Err(Error::new(expr.span, ErrorKind::Continue)),
        }
    }
}

// Helpers
impl ExecutionContext<'_> {
    fn find_var(&mut self, name: &Spanned<String>) -> ALResult<&mut Value> {
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

    fn find_type_def(&mut self, type_id: &Spanned<TypeID>) -> ALResult<TypeDef> {
        match &type_id.value {
            TypeID::Int => Ok(TypeDef::PrimitiveInt.into()),
            TypeID::Float => Ok(TypeDef::PrimitiveFloat.into()),
            TypeID::String => Ok(TypeDef::PrimitiveString.into()),
            TypeID::Bool => Ok(TypeDef::PrimitiveBool.into()),
            TypeID::Void => Ok(TypeDef::Void.into()),

            TypeID::User(name) => {
                let type_def = self.public_types.get(name).cloned();

                type_def.ok_or(Error::new(
                    type_id.span,
                    ErrorKind::TypeNotFound(name.clone()),
                ))
            }
        }
    }
}
