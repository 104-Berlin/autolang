use std::fmt::Display;

use miette::{miette, Context, LabeledSpan, SourceSpan};
use virtual_machine::{
    instruction::{
        args::{arg20::Arg20, jump_cond::JumpCondition, logical_operator::LogicalOperator},
        Instruction,
    },
    register::Register,
};

use crate::{
    compiler::compiler_context::{Buildable, CompilerContext, VarLocation},
    spanned::{SpanExt, Spanned, WithSpan},
    tokenizer::literal::Literal,
    ALResult,
};

use super::{
    binary_expression::{BinaryExpression, BinaryOperator},
    type_def::TypeID,
};

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
            Expr::Dot { .. } => todo!(),
            Expr::FunctionCall(_, _) => todo!(),
            Expr::Binary(bin) => Self::compile_binary_expression(builder, bin),
            Expr::Literal(val) => Self::compile_literal(builder, val),
            Expr::StructLiteral(_, _) => todo!(),
            Expr::Variable(var) => Self::compile_var_expr(builder, var),
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
            Expr::Return(_) => todo!(),
            Expr::Break => match builder.get_break_block() {
                Some(block) => builder.build_unconditional_jump(block, self.span),
                None => Err(miette!(
                    labels = vec![LabeledSpan::at(span, "here")],
                    "Break outside of loop"
                )),
            },
            Expr::Continue => todo!(),
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
            Expr::Binary(_) => unimplemented!(),
            Expr::Literal(_) => unimplemented!(),
            Expr::StructLiteral(_, _) => unimplemented!(),
            Expr::Variable(name) => {
                let var = builder.find_var(name).ok_or(miette!(
                    labels = vec![LabeledSpan::at(name.span, "here")],
                    "Variable not found"
                ))?;
                todo!()
            }
            Expr::Assignment(_, _) => unimplemented!(),
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
            Expr::Loop(_) => unimplemented!(),
            Expr::Block(_, return_expr) => return_expr
                .as_ref()
                .map(|e| e.guess_return_type(builder))
                .unwrap_or(Ok(TypeID::Void.with_span(own_span))),
            Expr::Return(expr) => expr
                .as_ref()
                .map(|e| e.guess_return_type(builder))
                .unwrap_or(Ok(TypeID::Void.with_span(own_span))),
            Expr::Break => unimplemented!(),
            Expr::Continue => unimplemented!(),
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
        _typ: &Option<Spanned<TypeID>>,
        assign: &Spanned<Expr>,
    ) -> ALResult<()> {
        assign.build(builder)?;
        // We need to register the variable in the symbol table

        miette::Context::wrap_err(builder.build_local_var(symbol), "Building Let")
    }

    fn compile_var_expr(builder: &mut CompilerContext, var: &Spanned<String>) -> ALResult<()> {
        // We need to check if it is a local variable or a global variable
        // fn test() {
        //    let a = 10;
        //    {
        //        let b = a;
        //    }
        // }
        //
        // 0: BP
        // 1: A
        // 2: B

        match builder.find_var(var) {
            Some(VarLocation::Local(offset)) => {
                todo!()
            }
            Some(VarLocation::Global(_addr)) => todo!("Global variables are not supported yet"),
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
        builder.build_conditional_jump(if_block, JumpCondition::NotZero, if_span_complete)?;
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

    fn compile_binary_expression(
        builder: &mut CompilerContext,
        bin: &Spanned<BinaryExpression>,
    ) -> ALResult<()> {
        // Load RHS into RA1 and LHS into RA2
        bin.lhs.build(builder)?;
        builder
            .build_instruction(
                Instruction::Move {
                    dst: Register::RA2.into(),
                    src: Register::RA1.into(),
                }
                .with_span(bin.span),
            )
            .wrap_err("Building Binary Expression")?;
        bin.rhs.build(builder)?;

        let op = &bin.op.value;
        let op_span = bin.op.span;

        match op {
            BinaryOperator::Add => builder
                .build_instruction(
                    Instruction::Add {
                        dst: Register::RA1,
                        lhs: Register::RA2.into(),
                        rhs: Register::RA1.into(),
                    }
                    .with_span(bin.span),
                )
                .wrap_err("Building (Add) Binary Expression")?,

            BinaryOperator::Assign => todo!(),
            BinaryOperator::Substract => todo!(),
            BinaryOperator::Multiply => todo!(),
            BinaryOperator::Divide => todo!(),
            BinaryOperator::And => todo!(),
            BinaryOperator::Or => todo!(),
            BinaryOperator::Equal => {
                Self::compile_compare(builder, LogicalOperator::EQ.with_span(op_span))?
            }
            BinaryOperator::NotEqual => {
                Self::compile_compare(builder, LogicalOperator::NE.with_span(op_span))?
            }
            BinaryOperator::LessThan => {
                Self::compile_compare(builder, LogicalOperator::LT.with_span(op_span))?
            }
            BinaryOperator::LessThanOrEqual => {
                Self::compile_compare(builder, LogicalOperator::LE.with_span(op_span))?
            }
            BinaryOperator::GreaterThan => {
                Self::compile_compare(builder, LogicalOperator::GT.with_span(op_span))?
            }
            BinaryOperator::GreaterThanOrEqual => {
                Self::compile_compare(builder, LogicalOperator::GE.with_span(op_span))?
            }
        };

        Ok(().with_span(bin.span))
    }

    /// Requires RA2 = lhs and RA1 = rhs
    fn compile_compare(
        builder: &mut CompilerContext,
        operator: Spanned<LogicalOperator>,
    ) -> ALResult<()> {
        builder
            .build_instruction(
                Instruction::Compare {
                    lhs: Register::RA2.into(),
                    rhs: Register::RA1.into(),
                }
                .with_span(operator.span),
            )
            .wrap_err("Building Compare")?;
        // Load the result of the comparison into RA1
        builder
            .build_instruction(
                Instruction::LoadBool {
                    dst: Register::RA1,
                    op: operator.value,
                }
                .with_span(operator.span),
            )
            .wrap_err("Building Compare")?;

        Ok(().with_span(operator.span))
    }

    fn compile_literal(builder: &mut CompilerContext, literal: &Spanned<Literal>) -> ALResult<()> {
        match **literal {
            // This is very bad. We are currently converting a i64 to an u32 (Arg20)
            Literal::NumberInt(val) => builder
                .build_instruction(
                    Instruction::Imm {
                        dst: Register::RA1,
                        value: Arg20(val as u32),
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
                        value: Arg20(if val { 1 } else { 0 }),
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
        // We need to push the arguments to the stack
        for arg in args {
            arg.build(builder)?; // Result should be in RA1
            builder
                .build_instruction(Instruction::Push(Register::RA1.into()).with_span(arg.span))
                .wrap_err("Building Function Call")?;
        }

        // Push next address to stack for returning back to the function
        builder
            .build_instruction(Instruction::Push(Register::PC.into()).with_span(callee.span.next()))
            .wrap_err("Building Function Call")?;

        // Push last base pointer to stack
        builder
            .build_instruction(Instruction::Push(Register::BP.into()).with_span(callee.span.next()))
            .wrap_err("Building Function Call")?;

        Ok(().with_span(callee.span))
    }
}
