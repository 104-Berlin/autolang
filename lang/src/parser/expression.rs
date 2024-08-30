use std::fmt::Display;

use miette::{miette, Context, Error, IntoDiagnostic, LabeledSpan};
use virtual_machine::{
    instruction::{
        args::{arg20::Arg20, jump_cond::JumpCondition, logical_operator::LogicalOperator},
        Instruction,
    },
    program_builder::{Buildable, ProgramBuilder},
    register::Register,
};

use crate::{error::VMToMietteError, spanned::Spanned, tokenizer::literal::Literal};

use super::{
    binary_expression::{BinaryExpression, BinaryOperator},
    type_def::TypeID,
};

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
    type Error = Error;

    fn build(
        &self,
        builder: &mut virtual_machine::program_builder::ProgramBuilder,
    ) -> Result<(), Self::Error> {
        let expr = &self.value;
        let span = self.span;
        match expr {
            Expr::Dot { .. } => todo!(),
            Expr::FunctionCall(callee, args) => todo!(),
            Expr::Binary(bin) => Self::compile_binary_expression(builder, bin),
            Expr::Literal(val) => Self::compile_literal(builder, val),
            Expr::StructLiteral(_, _) => todo!(),
            Expr::Variable(var) => Self::compile_var_expr(builder, var),
            Expr::Assignment(_, _) => todo!(),
            Expr::Let(symbol, typ, assign) => {
                assign.build(builder)?;
                builder
                    .build_instruction(Instruction::Push(Register::RA1.into()))
                    .into_diagnostic()
                    .wrap_err("Building Let")
            }
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
                Some(block) => builder.build_unconditional_jump(block).to_miette_error(),
                None => Err(miette!(
                    labels = vec![LabeledSpan::at(span, "here")],
                    "Break outside of loop"
                )),
            },
            Expr::Continue => todo!(),
        }
    }
}

impl Spanned<Expr> {
    fn compile_block_expr(
        builder: &mut ProgramBuilder,
        statements: &[Spanned<Expr>],
        return_expr: Option<&Spanned<Expr>>,
    ) -> Result<(), Error> {
        for statement in statements {
            statement.build(builder)?;
        }
        if let Some(return_expr) = return_expr {
            return_expr.build(builder)?;
        }
        Ok(())
    }

    fn compile_var_expr(builder: &mut ProgramBuilder, var: &Spanned<String>) -> Result<(), Error> {
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

        unimplemented!("Compile Var Expr")
    }

    fn compile_if_expr(
        builder: &mut ProgramBuilder,
        if_cond: &IfCondition,
        else_expr: Option<&Spanned<Expr>>,
    ) -> Result<(), Error> {
        let if_block = builder.append_block(Some("if_block")); // format!("if_{}", if_block.0.span.offset());
        let if_end_block = builder.append_block(Some("if_end_block")); //  format!("if_end_{}", if_block.0.span.offset());

        let else_block = else_expr
            .map(|_| builder.append_block(Some("else_block")))
            .unwrap_or(if_end_block);

        // Condition
        if_cond.0.build(builder)?; // Bool value in RA1

        // Jump to if block
        builder
            .build_conditional_jump(if_block, JumpCondition::NotZero)
            .to_miette_error()?;
        // Jump to else block or end of if in case of no else block
        builder
            .build_unconditional_jump(else_block)
            .to_miette_error()?;

        // If block
        builder.block_insertion_point(if_block).to_miette_error()?;
        if_cond.1.build(builder)?;
        // Append jump to end
        builder
            .build_unconditional_jump(if_end_block)
            .to_miette_error()?;

        // Else block
        if let Some(else_expr) = else_expr {
            // This is the appened else block. So not the if_end_block
            builder
                .block_insertion_point(else_block)
                .to_miette_error()?;
            else_expr.build(builder)?;
            builder
                .build_unconditional_jump(if_end_block)
                .to_miette_error()?;
        }

        builder
            .block_insertion_point(if_end_block)
            .to_miette_error()?;

        Ok(())
    }

    fn compile_binary_expression(
        builder: &mut ProgramBuilder,
        bin: &BinaryExpression,
    ) -> Result<(), Error> {
        // Load RHS into RA1 and LHS into RA2
        bin.lhs.build(builder)?;
        builder
            .build_instruction(Instruction::Copy {
                dst: Register::RA2,
                src: Register::RA1,
            })
            .into_diagnostic()
            .wrap_err("Building Binary Expression")?;
        bin.rhs.build(builder)?;

        match *bin.op {
            BinaryOperator::Add => builder
                .build_instruction(Instruction::Add {
                    dst: Register::RA1,
                    lhs: Register::RA2.into(),
                    rhs: Register::RA1.into(),
                })
                .into_diagnostic()
                .wrap_err("Building (Add) Binary Expression")?,

            BinaryOperator::Assign => todo!(),
            BinaryOperator::Substract => todo!(),
            BinaryOperator::Multiply => todo!(),
            BinaryOperator::Divide => todo!(),
            BinaryOperator::And => todo!(),
            BinaryOperator::Or => todo!(),
            BinaryOperator::Equal => Self::compile_compare(builder, LogicalOperator::EQ)?,
            BinaryOperator::NotEqual => Self::compile_compare(builder, LogicalOperator::NE)?,
            BinaryOperator::LessThan => Self::compile_compare(builder, LogicalOperator::LT)?,
            BinaryOperator::LessThanOrEqual => Self::compile_compare(builder, LogicalOperator::LE)?,
            BinaryOperator::GreaterThan => Self::compile_compare(builder, LogicalOperator::GT)?,
            BinaryOperator::GreaterThanOrEqual => {
                Self::compile_compare(builder, LogicalOperator::GE)?
            }
        };

        Ok(())
    }

    /// Requires RA2 = lhs and RA1 = rhs
    fn compile_compare(
        builder: &mut ProgramBuilder,
        operator: LogicalOperator,
    ) -> Result<(), Error> {
        builder
            .build_instruction(Instruction::Compare {
                lhs: Register::RA2.into(),
                rhs: Register::RA1.into(),
            })
            .into_diagnostic()
            .wrap_err("Building Compare")?;
        // Load the result of the comparison into RA1
        builder
            .build_instruction(Instruction::LoadBool {
                dst: Register::RA1,
                op: operator,
            })
            .into_diagnostic()
            .wrap_err("Building Compare")?;

        Ok(())
    }

    fn compile_literal(
        builder: &mut ProgramBuilder,
        literal: &Spanned<Literal>,
    ) -> Result<(), Error> {
        match **literal {
            // This is very bad. We are currently converting a i64 to an u32 (Arg20)
            Literal::NumberInt(val) => builder
                .build_instruction(Instruction::Imm {
                    dst: Register::RA1,
                    value: Arg20(val as u32),
                })
                .into_diagnostic()
                .wrap_err("Building Literal"),
            Literal::NumberFloat(_) => todo!("Floats are not supported yet"),
            Literal::String(_) => todo!("Strings are not supported yet"),
            Literal::Bool(val) => builder
                .build_instruction(Instruction::Imm {
                    dst: Register::RA1,
                    value: Arg20(if val { 1 } else { 0 }),
                })
                .into_diagnostic()
                .wrap_err("Building Literal"),
        }
    }

    fn compile_loop(builder: &mut ProgramBuilder, body: &Spanned<Expr>) -> Result<(), Error> {
        let loop_block = builder.append_block(Some("loop_block"));
        let end_block = builder.append_block(Some("end_block"));

        // Here could be a condition

        // Set break and continue block
        builder.set_break_block(end_block);
        builder.set_continue_block(loop_block);

        // Build block
        builder
            .block_insertion_point(loop_block)
            .to_miette_error()?;
        body.build(builder)?;

        // Jump back to loop block
        builder
            .build_unconditional_jump(loop_block)
            .to_miette_error()?;

        // Reset the break and continue block
        builder.pop_break_block();
        builder.pop_continuer_block();

        builder.block_insertion_point(end_block).to_miette_error()?;

        Ok(())
    }

    fn compile_function_call(
        builder: &mut ProgramBuilder,
        callee: &Spanned<String>,
        args: &[Spanned<Expr>],
    ) -> Result<(), Error> {
        // We need to push the arguments to the stack
        for arg in args {
            arg.build(builder)?; // Result should be in RA1
            builder
                .build_instruction(Instruction::Push(Register::RA1.into()))
                .into_diagnostic()
                .wrap_err("Building Function Call")?;
        }

        // Push next address to stack for returning back to the function
        builder
            .build_instruction(Instruction::Push(Register::PC.into()))
            .into_diagnostic()
            .wrap_err("Building Function Call")?;

        // Push last base pointer to stack
        builder
            .build_instruction(Instruction::Push(Register::BP.into()))
            .into_diagnostic()
            .wrap_err("Building Function Call")?;

        Ok(())
    }
}
