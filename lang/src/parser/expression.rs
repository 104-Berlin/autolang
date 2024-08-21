use std::fmt::Display;

use virtual_machine::{
    instruction::{
        args::{arg20::Arg20, jump_cond::JumpCondition, logical_operator::LogicalOperator},
        Instruction,
    },
    program_builder::{Buildable, UnresolvedInstruction},
    register::Register,
};

use crate::{error::ALError, spanned::Spanned, tokenizer::literal::Literal};

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
        // Pair of condition and block
        else_if_blocks: Vec<IfCondition>,
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
                else_if_blocks,
                else_block,
            } => {
                write!(f, "if {} {}", if_cond.value, if_block.value)?;
                for (condition, block) in else_if_blocks {
                    write!(f, " else if {} {}", condition.value, block.value)?;
                }
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

impl Buildable for Expr {
    type Error = ALError;

    fn build(
        &self,
        builder: &mut virtual_machine::program_builder::ProgramBuilder,
    ) -> Result<(), Self::Error> {
        match self {
            Expr::Dot { lhs, rhs } => todo!(),
            Expr::FunctionCall(_, _) => todo!(),
            Expr::Binary(bin) => {
                // Load RHS into RS1 and LHS into RS2
                bin.value.lhs.build(builder)?;
                builder.add_instruction(Instruction::Copy {
                    dst: Register::RS2,
                    src: Register::RS1,
                })?;
                bin.value.rhs.build(builder)?;

                match *bin.op {
                    BinaryOperator::Add => builder.add_instruction(Instruction::Add {
                        dst: Register::RS1,
                        lhs: Register::RS2.into(),
                        rhs: Register::RS1.into(),
                    })?,

                    BinaryOperator::Assign => todo!(),
                    BinaryOperator::Substract => todo!(),
                    BinaryOperator::Multiply => todo!(),
                    BinaryOperator::Divide => todo!(),
                    BinaryOperator::And => todo!(),
                    BinaryOperator::Or => todo!(),
                    BinaryOperator::Equal => {
                        // Make comparison
                        builder.add_instruction(Instruction::Compare {
                            lhs: Register::RS2.into(),
                            rhs: Register::RS1.into(),
                        })?;
                        // Load the result of the comparison into RS1
                        builder.add_instruction(Instruction::LoadBool {
                            dst: Register::RS1,
                            op: LogicalOperator::EQ,
                        })?;
                    }
                    BinaryOperator::NotEqual => {
                        // Make comparison
                        builder.add_instruction(Instruction::Compare {
                            lhs: Register::RS2.into(),
                            rhs: Register::RS1.into(),
                        })?;
                        // Load the result of the comparison into RS1
                        builder.add_instruction(Instruction::LoadBool {
                            dst: Register::RS1,
                            op: LogicalOperator::NE,
                        })?;
                    }
                    BinaryOperator::LessThan => {
                        // Make comparison
                        builder.add_instruction(Instruction::Compare {
                            lhs: Register::RS2.into(),
                            rhs: Register::RS1.into(),
                        })?;
                        // Load the result of the comparison into RS1
                        builder.add_instruction(Instruction::LoadBool {
                            dst: Register::RS1,
                            op: LogicalOperator::LT,
                        })?;
                    }
                    BinaryOperator::LessThanOrEqual => {
                        // Make comparison
                        builder.add_instruction(Instruction::Compare {
                            lhs: Register::RS2.into(),
                            rhs: Register::RS1.into(),
                        })?;
                        // Load the result of the comparison into RS1
                        builder.add_instruction(Instruction::LoadBool {
                            dst: Register::RS1,
                            op: LogicalOperator::LE,
                        })?;
                    }
                    BinaryOperator::GreaterThan => {
                        // Make comparison
                        builder.add_instruction(Instruction::Compare {
                            lhs: Register::RS2.into(),
                            rhs: Register::RS1.into(),
                        })?;
                        // Load the result of the comparison into RS1
                        builder.add_instruction(Instruction::LoadBool {
                            dst: Register::RS1,
                            op: LogicalOperator::GT,
                        })?;
                    }
                    BinaryOperator::GreaterThanOrEqual => {
                        // Make comparison
                        builder.add_instruction(Instruction::Compare {
                            lhs: Register::RS2.into(),
                            rhs: Register::RS1.into(),
                        })?;
                        // Load the result of the comparison into RS1
                        builder.add_instruction(Instruction::LoadBool {
                            dst: Register::RS1,
                            op: LogicalOperator::GE,
                        })?;
                    }
                };
            }
            Expr::Literal(val) => match **val {
                // This is very bad. We are currently converting a i64 to an u32 (Arg20)
                Literal::NumberInt(val) => builder.add_instruction(Instruction::Imm {
                    dst: Register::RS1,
                    value: Arg20(val as u32),
                })?,
                Literal::NumberFloat(_) => todo!("Floats are not supported yet"),
                Literal::String(_) => todo!("Strings are not supported yet"),
                Literal::Bool(_) => todo!("Bools are not supported yet"),
            },
            Expr::StructLiteral(_, _) => todo!(),
            Expr::Variable(_) => todo!(),
            Expr::Assignment(_, _) => todo!(),
            Expr::Let(symbol, typ, assign) => {
                assign.build(builder)?;
                builder.add_instruction(Instruction::Push(Register::RS1.into()))?;
            }
            Expr::IfExpression {
                if_block,
                else_if_blocks,
                else_block,
            } => {
                let if_label = format!("if_{}", if_block.0.span);
                let if_end_label = format!("if_end_{}", if_block.0.span);

                if_block.0.build(builder)?; // Bool value in RS1

                builder.add_unresolved(
                    Instruction::Jump {
                        cond: JumpCondition::NotZero, // If the comparision was true (e.a RS1 > 0)
                        offset: Arg20(0),             // What needs to go in here???
                    },
                    if_label.clone(),
                );

                for else_if_block in else_if_blocks {
                    let else_if_label = format!("else_if_{}", else_if_block.0.span);
                    else_if_block.0.build(builder)?; // Bool value in RS1

                    builder.add_unresolved(
                        Instruction::Jump {
                            cond: JumpCondition::NotZero, // If the comparision was true (e.a RS1 > 0)
                            offset: Arg20(0),             // What needs to go in here???
                        },
                        else_if_label.clone(),
                    );
                }

                let end = if let Some(else_block) = else_block {
                    let else_label = format!("else_{}", else_block.span);
                    else_label.clone()
                } else {
                    if_end_label.clone()
                };

                builder.add_unresolved(
                    Instruction::Jump {
                        cond: JumpCondition::Always,
                        offset: Arg20(0),
                    },
                    end,
                );

                let build_body =
                    |builder: &mut virtual_machine::program_builder::ProgramBuilder,
                     body: &Box<Spanned<Expr>>,
                     label: String|
                     -> Result<(), ALError> {
                        builder.add_label(label);
                        body.build(builder)?;
                        builder.add_unresolved(
                            Instruction::Jump {
                                cond: JumpCondition::Always,
                                offset: Arg20(0),
                            },
                            if_end_label.clone(),
                        );

                        Ok(())
                    };

                // if block
                build_body(builder, &if_block.1, if_label)?;

                for else_if_block in else_if_blocks {
                    let else_if_label = format!("else_if_{}", else_if_block.0.span);
                    build_body(builder, &else_if_block.1, else_if_label)?;
                }

                if let Some(else_block) = else_block {
                    let else_label = format!("else_{}", else_block.span);
                    build_body(builder, &else_block, else_label)?;
                }

                builder.add_label(if_end_label);
            }
            Expr::Loop(_) => todo!(),
            Expr::Block(statements, return_expr) => {
                for statement in statements {
                    statement.build(builder)?;
                }
                if let Some(return_expr) = return_expr {
                    return_expr.build(builder)?;
                }
            }
            Expr::Return(_) => todo!(),
            Expr::Break => todo!(),
            Expr::Continue => todo!(),
        }
        Ok(())
    }
}
