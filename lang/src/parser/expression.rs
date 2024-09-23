use std::fmt::Display;

use miette::{miette, Context, LabeledSpan, SourceSpan};
use virtual_machine::{
    instruction::{
        args::{
            arg_n::Arg20, jump_cond::JumpCondition,
            register_or_register_pointer::RegisterOrRegisterPointer,
            register_pointer::RegisterPointer,
        },
        Instruction,
    },
    register::Register,
};

use crate::{
    compiler::{
        compiler_context::{Buildable, CompilerContext, VarLocation},
        unresolved_instruction::{Unresolved, UnresolvedInstruction},
    },
    prelude::ArgumentDecl,
    spanned::{SpanExt, Spanned, WithSpan},
    tokenizer::literal::Literal,
    ALResult,
};

use super::{binary_expression::BinaryExpression, type_def::TypeID};

/// (Condition, Block)
pub type IfCondition = (Box<Spanned<Expr>>, Box<Spanned<Expr>>);

#[derive(Debug, Clone)]
pub enum DotExpr {
    FunctionCall(Spanned<String>, Vec<Spanned<Expr>>),
    Variable(Spanned<String>),
}

// Something that can yield a value
#[derive(Debug, Clone)]
pub enum Expr {
    /// A connected series of expressions combined with dot.
    /// # Example
    /// ```rs
    /// a.b().c.d(e, f)
    /// ```
    Dot {
        lhs: Box<Spanned<Expr>>,
        rhs: Spanned<DotExpr>,
    },

    FunctionCall(Spanned<String>, Vec<Spanned<Expr>>),
    Binary(Spanned<BinaryExpression>),

    Literal(Spanned<Literal>),
    StructLiteral(Spanned<String>, Vec<(Spanned<String>, Spanned<Expr>)>),
    Variable(Spanned<String>),

    Assignment(Spanned<String>, Box<Spanned<Expr>>),

    Let(Spanned<String>, Option<Spanned<TypeID>>, Box<Spanned<Expr>>),

    IfExpression {
        if_block: IfCondition,
        else_block: Option<Box<Spanned<Expr>>>,
    },

    Loop(Box<Spanned<Expr>>),

    Block(Vec<Spanned<Expr>>, Option<Box<Spanned<Expr>>>),

    Lambda {
        args: Spanned<Vec<ArgumentDecl>>,
        return_type: Option<Spanned<TypeID>>, // None means we guess from the body. Otherwise the body must return the correct type
        body: Box<Spanned<Expr>>,
    },

    Return(Option<Box<Spanned<Expr>>>),
    Break,
    Continue,
}

impl From<DotExpr> for Expr {
    fn from(value: DotExpr) -> Self {
        match value {
            DotExpr::FunctionCall(name, args) => Expr::FunctionCall(name, args),
            DotExpr::Variable(name) => Expr::Variable(name),
        }
    }
}

impl Display for DotExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotExpr::FunctionCall(name, args) => {
                write!(
                    f,
                    "{}({})",
                    name.value,
                    args.iter()
                        .map(|a| a.value.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            DotExpr::Variable(name) => write!(f, "{}", name.value),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Dot { lhs, rhs } => write!(f, "{}.{}", lhs.value, rhs.value),
            Expr::FunctionCall(name, vars) => write!(
                f,
                "{}({})",
                name.value,
                vars.iter()
                    .map(|v| format!("{}", v.value))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Binary(expr) => {
                write!(
                    f,
                    "{} {} {}",
                    expr.value.lhs.value, expr.value.op.value, expr.value.rhs.value
                )
            }
            Expr::Assignment(var, expr) => write!(f, "{} = {}", var.value, expr.value),
            Expr::Let(var, type_id, assign) => match &type_id {
                Some(type_id) => {
                    write!(f, "let {}: {} = {}", var.value, type_id.value, assign.value)
                }
                None => write!(f, "let {} = {}", var.value, assign.value),
            },
            Expr::Literal(literal) => write!(f, "{}", literal.value),
            Expr::StructLiteral(name, fields) => {
                write!(f, "{} {{", name.value)?;
                for (field_name, field_expr) in fields.iter() {
                    write!(f, "{}: {}, ", field_name.value, field_expr.value)?;
                }
                write!(f, "}}")
            }
            Expr::Variable(name) => write!(f, "{}", name.value),
            Expr::IfExpression {
                if_block: (if_cond, if_block),
                else_block,
            } => {
                write!(f, "if {} {}", if_cond.value, if_block.value)?;
                if let Some(else_block) = else_block {
                    write!(f, " else {}", else_block.value)?;
                }
                Ok(())
            }
            Expr::Block(expr, return_expr) => {
                write!(f, "{{")?;
                for e in expr {
                    write!(f, "{}, ", e.value)?;
                }
                if let Some(return_expr) = return_expr {
                    write!(f, "{}", return_expr.value)?;
                }
                write!(f, "}}")
            }
            Expr::Loop(expr) => write!(f, "loop {}", expr.value),
            Expr::Lambda {
                args,
                return_type,
                body,
            } => {
                write!(f, "L(")?;
                for (i, arg) in args.value.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", arg.0.value, arg.1.value)?;
                }
                write!(f, ")")?;
                if let Some(return_type) = return_type {
                    write!(f, " -> {}", return_type.value)?;
                }
                write!(f, " {}", body.value)
            }
            Expr::Return(expr) => write!(
                f,
                "return{}",
                expr.as_ref()
                    .map(|e| format!(" {}", e.value))
                    .unwrap_or(";".to_string())
            ),
            Expr::Break => write!(f, "break"),
            Expr::Continue => write!(f, "continue"),
        }
    }
}

impl Buildable for Spanned<Expr> {
    fn build(&self, builder: &mut CompilerContext) -> ALResult<()> {
        let expr = &self.value;
        let span = self.span;
        match expr {
            Expr::Dot { .. } => todo!(), // With structs
            Expr::FunctionCall(callee, args) => Self::compile_function_call(builder, callee, args),
            Expr::Binary(bin) => bin.build(builder),
            Expr::Literal(val) => Self::compile_literal(builder, val),
            Expr::StructLiteral(_, _) => todo!(), // With structs
            Expr::Variable(var) => Self::compile_var_expr(builder, var, span),
            Expr::Assignment(_, _) => todo!(),
            Expr::Let(symbol, typ, assign) => Self::compile_let_expr(builder, symbol, typ, assign),
            Expr::IfExpression {
                if_block,
                else_block,
            } => Self::compile_if_expr(builder, if_block, else_block.as_ref().map(AsRef::as_ref)),
            Expr::Loop(body) => Self::compile_loop(builder, body),
            Expr::Block(statements, return_expr) => Self::compile_block_expr(
                builder,
                statements,
                return_expr.as_ref().map(AsRef::as_ref),
            ),
            Expr::Lambda { .. } => {
                todo!()
            }
            Expr::Return(_) => builder.build_return(self.span),
            Expr::Break => match builder.get_break_block() {
                Some(block) => builder.build_unconditional_jump(block, self.span),
                None => Err(miette!(
                    labels = vec![LabeledSpan::at(span, "here")],
                    "Break outside of loop"
                )),
            },
            Expr::Continue => match builder.get_continue_block() {
                Some(block) => builder.build_unconditional_jump(block, self.span),
                None => Err(miette!(
                    labels = vec![LabeledSpan::at(span, "here")],
                    "Continue outside of loop"
                )),
            },
        }
    }
}

// Guessing return types

impl DotExpr {
    pub fn guess_return_type(&self, builder: &mut CompilerContext) -> TypeID {
        match self {
            DotExpr::FunctionCall(_, _) => todo!(),
            DotExpr::Variable(name) => todo!(),
        }
    }
}

impl Spanned<Expr> {
    /// Span will be the expression that produces the value
    pub fn guess_return_type(&self, builder: &mut CompilerContext) -> ALResult<TypeID> {
        let own_span = self.span;

        match &self.value {
            Expr::Dot { .. } => unimplemented!(),
            Expr::FunctionCall(_, _) => unimplemented!(),
            Expr::Binary(bin) => bin.guess_return_type(builder),
            Expr::Literal(lit) => {
                let typ = match lit.value {
                    Literal::NumberInt(_) => TypeID::Int,
                    Literal::NumberFloat(_) => TypeID::Float,
                    Literal::String(_) => TypeID::String,
                    Literal::Bool(_) => TypeID::Bool,
                };
                Ok(typ.with_span(own_span))
            }
            Expr::StructLiteral(_, _) => unimplemented!(),
            Expr::Variable(name) => {
                let var = builder.find_var(name).ok_or(miette!(
                    labels = vec![LabeledSpan::at(name.span, "here")],
                    "Variable not found"
                ))?;
                Ok(var.1.with_span(name.span))
            }
            Expr::Assignment(_, _) => Ok(TypeID::Void.with_span(own_span)),
            Expr::Let(_, _, _) => Ok(TypeID::Void.with_span(own_span)),
            Expr::IfExpression {
                if_block,
                else_block,
            } => {
                let if_type = if_block.1.guess_return_type(builder)?;
                if let Some(else_type) = else_block
                    .as_ref()
                    .map(|e| e.guess_return_type(builder))
                    .transpose()?
                {
                    // Two different types of the if and else block
                    if if_type.value != else_type.value {
                        Err(miette!(
                            labels = vec![
                                LabeledSpan::at(if_type.span, "if type"),
                                LabeledSpan::at(else_type.span, "else type")
                            ],
                            "If and else block have different return types"
                        ))
                    } else {
                        Ok(if_type)
                    }
                } else {
                    Ok(if_type)
                }
            }
            Expr::Loop(body) => body.guess_return_type(builder),
            Expr::Block(_, return_expr) => return_expr
                .as_ref()
                .map(|e| e.guess_return_type(builder))
                .unwrap_or(Ok(TypeID::Void.with_span(own_span))),
            Expr::Lambda {
                args,
                return_type,
                body,
            } => Ok(TypeID::Function(
                args.value
                    .iter()
                    .map(|arg| arg.1.value.clone())
                    .collect::<Vec<_>>(),
                Box::new(
                    return_type
                        .as_ref()
                        .map(|t| t.value.clone())
                        .unwrap_or_else(|| {
                            body.guess_return_type(builder)
                                .wrap_err("Guessing return type of lambda")
                                .unwrap()
                                .value
                        }),
                ),
            )
            .with_span(own_span)),
            Expr::Return(expr) => expr
                .as_ref()
                .map(|e| e.guess_return_type(builder))
                .unwrap_or(Ok(TypeID::Void.with_span(own_span))),
            Expr::Break => Ok(TypeID::Void.with_span(own_span)),
            Expr::Continue => Ok(TypeID::Void.with_span(own_span)),
        }
    }
}

impl Spanned<Expr> {
    fn compile_block_expr(
        builder: &mut CompilerContext,
        statements: &[Spanned<Expr>],
        return_expr: Option<&Spanned<Expr>>,
    ) -> ALResult<()> {
        builder.push_scope();
        let mut span: Option<SourceSpan> = None;
        for statement in statements {
            statement.build(builder)?;
            match span {
                Some(ref mut span) => *span = span.union(&statement.span),
                None => span = Some(statement.span),
            }
        }
        if let Some(return_expr) = return_expr {
            return_expr.build(builder)?;
            match span {
                Some(ref mut span) => *span = span.union(&return_expr.span),
                None => span = Some(return_expr.span),
            }
        }
        builder.pop_scope();
        Ok(().with_span(span.unwrap_or(SourceSpan::from(0..0))))
    }

    fn compile_let_expr(
        builder: &mut CompilerContext,
        symbol: &Spanned<String>,
        typ: &Option<Spanned<TypeID>>,
        assign: &Spanned<Expr>,
    ) -> ALResult<()> {
        // First we need to check to types
        let expr_type = assign.guess_return_type(builder)?;
        let types_match = typ
            .as_ref()
            .map(|t| t.value == expr_type.value)
            .unwrap_or(true);

        if !types_match {
            return Err(miette!(
                labels = vec![
                    LabeledSpan::at(symbol.span, "here"),
                    LabeledSpan::at(typ.as_ref().unwrap().span, "expected type"),
                    LabeledSpan::at(assign.span, "assigned type")
                ],
                "Type mismatch"
            ));
        }

        assign.build(builder)?;
        // We need to register the variable in the symbol table

        builder
            .build_local_var(symbol, expr_type.value)
            .wrap_err("Building Let")
    }

    fn compile_var_expr(
        builder: &mut CompilerContext,
        var: &Spanned<String>,
        span: SourceSpan,
    ) -> ALResult<()> {
        match builder.find_var(var) {
            Some((VarLocation::Local(offset), _)) => {
                println!("Found local variable at offset {}", offset);

                // For now we load the value from the base_pointer + offset into RA1
                let bp = RegisterPointer {
                    register: Register::BP,
                    offset: offset as u8,
                };

                builder.build_instruction(
                    Instruction::Move {
                        dst: Register::RA1.into(),
                        src: RegisterOrRegisterPointer::RegisterPointer(bp),
                    }
                    .with_span(span),
                )?;
                Ok(().with_span(span))
            }
            Some((VarLocation::Global(_addr), _typ)) => {
                todo!("Global variables are not supported yet")
            }
            None => Err(miette!(
                labels = vec![LabeledSpan::at(var.span, "here")],
                "Variable not found"
            )),
        }
    }

    fn compile_if_expr(
        builder: &mut CompilerContext,
        if_cond: &IfCondition,
        else_expr: Option<&Spanned<Expr>>,
    ) -> ALResult<()> {
        let if_span_complete = if_cond.0.span.union(&if_cond.1.span);
        let if_span_block = if_cond.1.span;
        let if_span_cond = if_cond.0.span;
        let end_span = else_expr.map(|e| e.span).unwrap_or(if_span_complete).next();

        let if_block = builder.append_block(Some("if_block")); // format!("if_{}", if_block.0.span.offset());
        let if_end_block = builder.append_block(Some("if_end_block")); //  format!("if_end_{}", if_block.0.span.offset());

        let else_block = else_expr
            .map(|_| builder.append_block(Some("else_block")))
            .unwrap_or(if_end_block);

        // Condition
        if_cond.0.build(builder)?; // Bool value in RA1

        // Jump to if block
        builder.build_conditional_jump(if_block, JumpCondition::NotZero, if_span_cond)?;
        // Jump to else block or end of if in case of no else block
        builder.build_unconditional_jump(
            else_block,
            else_expr.map(|e| e.span).unwrap_or(if_span_complete),
        )?;

        // If block
        builder.block_insertion_point(if_block, if_span_block)?;
        if_cond.1.build(builder)?;
        // Append jump to end
        builder.build_unconditional_jump(if_end_block, if_span_block)?;

        // Else block
        if let Some(else_expr) = else_expr {
            // This is the appened else block. So not the if_end_block
            builder.block_insertion_point(else_block, else_expr.span)?;
            else_expr.build(builder)?;
            builder.build_unconditional_jump(if_end_block, else_expr.span)?;
        }

        builder.block_insertion_point(if_end_block, end_span)?;

        Ok(().with_span(if_span_complete.union(&end_span)))
    }

    fn compile_literal(builder: &mut CompilerContext, literal: &Spanned<Literal>) -> ALResult<()> {
        match **literal {
            // This is very bad. We are currently converting a i64 to an u32 (Arg20)
            Literal::NumberInt(val) => builder
                .build_instruction(
                    Instruction::Imm {
                        dst: Register::RA1,
                        value: Arg20::from(val as u32),
                    }
                    .with_span(literal.span),
                )
                .wrap_err("Building Literal"),
            Literal::NumberFloat(_) => todo!("Floats are not supported yet"),
            Literal::String(_) => todo!("Strings are not supported yet"),
            Literal::Bool(val) => builder
                .build_instruction(
                    Instruction::Imm {
                        dst: Register::RA1,
                        value: Arg20::from(if val { 1 } else { 0 }),
                    }
                    .with_span(literal.span),
                )
                .wrap_err("Building Literal"),
        }
    }

    fn compile_loop(builder: &mut CompilerContext, body: &Spanned<Expr>) -> ALResult<()> {
        let loop_block = builder.append_block(Some("loop_block"));
        let end_block = builder.append_block(Some("end_block"));

        // Here could be a condition

        // Set break and continue block
        builder.set_break_block(end_block);
        builder.set_continue_block(loop_block);

        // Build block
        builder.block_insertion_point(loop_block, body.span)?;
        body.build(builder)?;

        // Jump back to loop block
        builder.build_unconditional_jump(loop_block, body.span)?;

        // Reset the break and continue block
        builder.pop_break_block();
        builder.pop_continuer_block();

        builder.block_insertion_point(end_block, body.span)?;

        Ok(().with_span(body.span))
    }

    fn compile_function_call(
        builder: &mut CompilerContext,
        callee: &Spanned<String>,
        args: &[Spanned<Expr>],
    ) -> ALResult<()> {
        // Push next address to stack for returning back to the function
        builder
            .build_instruction(Instruction::Push(Register::PC.into()).with_span(callee.span.next()))
            .wrap_err("Building Function Call")?;

        // We need to push the arguments to the stack
        for arg in args {
            arg.build(builder)?; // Result should be in RA1
            builder
                .build_instruction(Instruction::Push(Register::RA1.into()).with_span(arg.span))
                .wrap_err("Building Function Call")?;
        }

        builder.build_instruction_unresolved(
            UnresolvedInstruction::Jump {
                cond: JumpCondition::Always,
                offset: Unresolved::Unresolved(callee.value.clone()),
            }
            .with_span(callee.span),
        )?;

        Ok(().with_span(callee.span))
    }
}
