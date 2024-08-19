use std::env::args;

use virtual_machine::{
    error::VMResult,
    instruction::{
        args::{arg20::Arg20, jump_cond::JumpCondition, register_or_literal::RegisterOrLiteral},
        Instruction,
    },
    program_builder::ProgramBuilder,
    register::Register,
};

fn main() -> VMResult<()> {
    let step_mode = args().nth(1).map(|s| s == "-s").unwrap_or(false);

    const SIZE_IN_BYTES: usize = 1024 * 1024;
    // Use this, because each entry of our array is a u32,
    // which is 4 bytes wide
    const SIZE_IN_4_BYTES: usize = SIZE_IN_BYTES / 4;

    let mut memory = vec![0u32; SIZE_IN_4_BYTES];
    memory[2999] = 12;
    prog_simple_loop(ProgramBuilder::new(memory))?
        .finish()
        .run(step_mode)?;
    Ok(())
}

#[allow(dead_code)]
fn prog_test(builder: ProgramBuilder) -> VMResult<ProgramBuilder> {
    builder
        .add_value(2999, 32)?
        .add_instruction(Instruction::Load(Register::RA1, Arg20(-2i32 as u32)))?
        .add_instruction(Instruction::Imm(Register::RA2, Arg20(-3i32 as u32)))?
        .add_instruction(Instruction::Add(
            Register::RA3,
            Register::RA1.into(),
            Register::RA2.into(),
        ))?
        .add_instruction(Instruction::Add(
            Register::RA4,
            Register::RA1.into(),
            RegisterOrLiteral::Literal(12),
        ))?
        .add_instruction(Instruction::Jump(
            JumpCondition::Always,
            Arg20(-5i32 as u32),
        ))
}

#[allow(dead_code)]
fn prog_simple_loop(builder: ProgramBuilder) -> VMResult<ProgramBuilder> {
    builder
        // Load loop count into ra1
        .add_instruction(Instruction::Imm(Register::RA1, Arg20(5)))?
        .add_instruction(Instruction::Add(
            Register::RA1,
            Register::RA1.into(),
            RegisterOrLiteral::Literal(-1i8 as u8),
        ))?
        .add_instruction(Instruction::Compare(
            Register::RA1,
            RegisterOrLiteral::Literal(0),
        ))?
        .add_instruction(Instruction::Jump(
            JumpCondition::NotZero,
            Arg20(-3i32 as u32),
        ))
}
