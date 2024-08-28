use std::env::args;

use virtual_machine::{
    error::VMResult,
    instruction::{
        args::{arg20::Arg20, jump_cond::JumpCondition, register_or_literal::RegisterOrLiteral},
        Instruction,
    },
    machine::Machine,
    program_builder::ProgramBuilder,
    register::Register,
};

fn main() -> VMResult<()> {
    let step_mode = args().nth(1).map(|s| s == "-s").unwrap_or(false);

    let mut builder = ProgramBuilder::default();
    prog_stack(&mut builder)?;

    Machine::default()
        .load_program(&builder.finish()?)?
        .run(step_mode)?;
    Ok(())
}

#[allow(dead_code)]
fn prog_test(builder: &mut ProgramBuilder) -> VMResult<()> {
    builder.add_value(2999, 32)?;

    builder.build_instruction(Instruction::Load {
        dst: Register::RA1,
        offset: Arg20(-2i32 as u32),
    })?;
    builder.build_instruction(Instruction::Imm {
        dst: Register::RA2,
        value: Arg20(-3i32 as u32),
    })?;
    builder.build_instruction(Instruction::Add {
        dst: Register::RA3,
        lhs: Register::RA1.into(),
        rhs: Register::RA2.into(),
    })?;
    builder.build_instruction(Instruction::Add {
        dst: Register::RA4,
        lhs: Register::RA1.into(),
        rhs: RegisterOrLiteral::Literal(12),
    })?;
    builder.build_instruction(Instruction::Jump {
        cond: JumpCondition::Always,
        offset: Arg20(-5i32 as u32),
    })?;

    Ok(())
}

#[allow(dead_code)]
fn prog_simple_loop(builder: &mut ProgramBuilder) -> VMResult<()> {
    // Load loop count into ra1
    builder.build_instruction(Instruction::Imm {
        dst: Register::RA1,
        value: Arg20(5),
    })?;
    builder.build_instruction(Instruction::Add {
        dst: Register::RA1,
        lhs: Register::RA1.into(),
        rhs: RegisterOrLiteral::Literal(-1i8 as u8),
    })?;
    builder.build_instruction(Instruction::Compare {
        lhs: Register::RA1.into(),
        rhs: RegisterOrLiteral::Literal(0),
    })?;
    builder.build_instruction(Instruction::Jump {
        cond: JumpCondition::NotZero,
        offset: Arg20(-3i32 as u32),
    })?;
    Ok(())
}

#[allow(dead_code)]
fn prog_stack(builder: &mut ProgramBuilder) -> VMResult<()> {
    builder.build_instruction(Instruction::Push(RegisterOrLiteral::Literal(32)))?;
    builder.build_instruction(Instruction::Pop(Register::RA1))?;
    Ok(())
}
