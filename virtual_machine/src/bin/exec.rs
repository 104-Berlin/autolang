use virtual_machine::{
    error::VMResult,
    instruction::{Arg20, InstructionWriter},
    opcode::OpCode,
    register::Register,
    sign_extend, Machine,
};

fn main() -> VMResult<()> {
    const SIZE_IN_BYTES: usize = 1024 * 1024;
    // Use this, because each entry of our array is a u32,
    // which is 4 bytes wide
    const SIZE_IN_4_BYTES: usize = SIZE_IN_BYTES / 4;

    let mut memory = vec![0u32; SIZE_IN_4_BYTES];
    memory[2999] = 32;
    memory[3000] = InstructionWriter::new(OpCode::Nop).finish();
    memory[3001] = InstructionWriter::new(OpCode::Load)
        .write(Register::RA1)
        .write(Arg20(sign_extend((-3i8 as u8) as u32, 8)))
        .finish();
    memory[3002] = InstructionWriter::new(OpCode::Imm)
        .write(Register::RA2)
        .write(Arg20(-3i16 as u32))
        .finish();

    let mut machine = Machine::new(memory);

    machine.run()?;

    println!("{}", machine.registers());

    Ok(())
}
