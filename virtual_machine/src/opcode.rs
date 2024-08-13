use crate::{
    error::{VMError, VMResult},
    instruction::InstructionPart,
};

/// # 6 Bit
#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Halt, // Stop the program
    Nop,  // Do nothing
    Load, // Load a value into a register
    Imm,  // Load an immediate value into a register
    Add,  // Add two numbers
}

impl InstructionPart for OpCode {
    const BIT_SIZE: u32 = 6;

    fn match_to_bytes(data: Self) -> u32 {
        data as u32
    }

    fn match_from_bytes(value: u32) -> VMResult<Self> {
        let value = value as u8;

        match value {
            0x0 => Ok(OpCode::Halt),
            0x1 => Ok(OpCode::Nop),
            0x2 => Ok(OpCode::Load),
            0x3 => Ok(OpCode::Imm),
            0x4 => Ok(OpCode::Add),
            _ => Err(VMError::InvalidOpCode(value)),
        }
    }
}
