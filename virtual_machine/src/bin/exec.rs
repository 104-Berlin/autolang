use virtual_machine::{
    error::VMResult,
    instruction::{Arg20, Instruction, RegisterOrLiteral},
    program_builder::ProgramBuilder,
    register::Register,
};

fn main() -> VMResult<()> {
    const SIZE_IN_BYTES: usize = 1024 * 1024;
    // Use this, because each entry of our array is a u32,
    // which is 4 bytes wide
    const SIZE_IN_4_BYTES: usize = SIZE_IN_BYTES / 4;

    let mut memory = vec![0u32; SIZE_IN_4_BYTES];
    memory[2999] = 12;
    let machine = ProgramBuilder::new(memory)
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
        .finish()
        .run()?;

    println!("{}", machine.registers());
    Ok(())
}
