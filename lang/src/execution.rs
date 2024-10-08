use std::collections::HashMap;

use miette::{miette, Context, Error, LabeledSpan, SourceSpan};
/// This Module is used to execute a program.
use value::Value;

use crate::{
    error::{ControllFlow, InvalidNumberOfArguments, TypeMismatch, TypeMismatchReason},
    module::Module,
    parser::{
        binary_expression::{BinaryExpression, BinaryOperator},
        expression::{DotExpr, Expr},
        function::FunctionDecl,
        structs::StructValue,
        type_def::{TypeDef, TypeID},
    },
    spanned::{SpanExt, Spanned},
    system_functions::{self, IntoSystem, System},
    tokenizer::literal::Literal,
    ALResult,
};

pub mod value;

pub struct ExecutionContext<'a> {
    pub span: SourceSpan,
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
            return Err(miette!("No main function found"));
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
            (None, None) => Err(miette!("Function '{}' not found", func_name.value)),
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
        call_span: SourceSpan,
        function: &Spanned<FunctionDecl>,
        arguments: Vec<ALResult<Value>>,
    ) -> ALResult<Value> {
        // Check for provided arguments
        if function.value.proto.value.arguments.value.len() != arguments.len() {
            return Err(InvalidNumberOfArguments {
                found: arguments.len(),
                expected: function.value.proto.value.arguments.value.len(),
                span: call_span,
            }
            .into());
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
                return Err(TypeMismatch {
                    found: value.value.type_id.clone(),
                    expected: arg_type.value.clone(),
                    reason: TypeMismatchReason::FunctionArgument,
                    span: value.span,
                }
                .into());
            }

            // Make spanned tuple of the variable name and the value
            // The Span will be the span of the expression which is the input for the function call
            let value = value.map_value(|val| (arg_name.value.clone(), val));

            scope.variables.push(value);
        }

        // Push scope for the body
        self.scopes.push(scope);

        let res = self.run_expr(&function.value.body).or_else(|err| {
            match err.downcast_ref::<ControllFlow>() {
                Some(ControllFlow::Return(val)) => Ok(Spanned::new(val.clone(), call_span)),
                _ => Err(err),
            }
        })?;

        // Pop the scope
        self.scopes.pop();

        if res.value.type_id != return_type {
            // Return types dont match
            return Err(TypeMismatch {
                found: res.value.type_id.clone(),
                expected: return_type,
                reason: TypeMismatchReason::FunctionReturn,
                span: res.span,
            }
            .into());
        }

        Ok(res)
    }

    fn run_expr(&mut self, expr: &Spanned<Expr>) -> ALResult<Value> {
        match &expr.value {
            Expr::Dot { lhs, rhs } => {
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
                                ).ok_or(miette!(
                                    labels = vec![LabeledSpan::at(expr.span, "here")],
                                    "Field not found",
                                ))
                            }
                            _ => Err(miette!("Can't access field of non-struct type")),
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
                    return Err(miette!(
                        labels = vec![LabeledSpan::at(name.span, "here")],
                        "Type is not a struct",
                    ));
                };

                let mut struct_value = StructValue::default();
                for struct_def_field in struct_def.fields.iter() {
                    let field = field_inits
                        .iter()
                        .find(|f| f.0.value == struct_def_field.value.0)
                        .map(|f| self.run_expr(&f.1))
                        .ok_or(miette!(
                            labels = vec![LabeledSpan::at(name.span, "here")],
                            "Field not initialized",
                        ))??;

                    // Handle invalid type
                    if field.value.type_id != struct_def_field.value.1 {
                        return Err(TypeMismatch {
                            found: field.value.type_id.clone(),
                            expected: struct_def_field.value.1.clone(),
                            reason: TypeMismatchReason::FunctionArgument,
                            span: field.span,
                        })
                        .wrap_err("Field initialization");
                    }
                    struct_value.push_field(field);
                }
                // Check if we try to initialize a field that is not in the struct
                for field in field_inits {
                    if !struct_def.fields.iter().any(|f| f.value.0 == field.0.value) {
                        return Err(miette!(
                            labels = vec![LabeledSpan::at(field.0.span, "here")],
                            "Field not found",
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
                if let Some(Some(v)) = self.scopes.last().map(|scope| {
                    scope
                        .variables
                        .iter()
                        .find(|var| var.value.0 == var_name.value)
                }) {
                    return Err(miette!(
                        labels = vec![
                            LabeledSpan::at(var_name.span, "this"),
                            LabeledSpan::at(v.span, "here")
                        ],
                        "Variable already defined",
                    ));
                }

                let span = assign.span;

                let value = self.run_expr(assign)?.value;

                if let Some(type_id) = type_id {
                    if value.type_id != type_id.value {
                        return Err(TypeMismatch {
                            found: value.type_id.clone(),
                            expected: type_id.value.clone(),
                            reason: TypeMismatchReason::VariableAssignment,
                            span,
                        }
                        .into());
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
                        return Err(miette!(
                            labels = vec![LabeledSpan::at(lhs.span, "here")],
                            "Left hand side of assignment must be a variable",
                        ));
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
                .map(|v| v.map_span(|_| lhs.span.union(&rhs.span)))
            }
            Expr::IfExpression {
                if_block: (condition, then_block),
                else_if_blocks,
                else_block,
            } => {
                let condition = self.run_expr(condition)?;
                let value = condition.value.as_bool().ok_or(TypeMismatch {
                    found: condition.value.type_id.clone(),
                    expected: TypeID::Bool,
                    reason: TypeMismatchReason::FunctionArgument,
                    span: condition.span,
                })?;

                if value {
                    return self.run_expr(then_block);
                }

                for (else_if_cond, else_if_block) in else_if_blocks {
                    let condition = self.run_expr(else_if_cond)?;
                    let value = condition.value.as_bool().ok_or(miette!(
                        labels = vec![LabeledSpan::at(else_if_cond.span, "here")],
                        "Condition must be a boolean",
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
                        let flow = err.downcast_ref::<ControllFlow>();
                        match flow {
                            Some(ControllFlow::Break) => {
                                break Ok(Spanned::new(Value::new_void(), expr.span))
                            }
                            Some(ControllFlow::Continue) => continue,
                            _ => return Err(err),
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
                Err(ControllFlow::Return(value.value).into())
            }
            Expr::Break => Err(ControllFlow::Break.into()),
            Expr::Continue => Err(ControllFlow::Continue.into()),
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

        Err(miette!(
            labels = vec![LabeledSpan::at(name.span, "here")],
            "Variable not found",
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

                type_def.ok_or(miette!(
                    labels = vec![LabeledSpan::at(type_id.span, "here")],
                    "Type not found",
                ))
            }
        }
    }
}
