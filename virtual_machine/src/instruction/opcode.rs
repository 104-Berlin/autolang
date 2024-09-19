use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    error::{VMError, VMResult},
    instruction::args::InstructionArg,
};

/// # 6 Bit
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Halt, // Stop the program
    Nop,  // Do nothing
    Imm,  // Load an immediate value into a register
    Add,  // Add two numbers

    Push, // Push a value onto the stack
    Pop,  // Pop a value from the stack

    Move, // Move a value from a register to a memory location

    Compare,  // Compare two numbers
    LoadBool, // Load a boolean from the condition register to have a bool type

    Jump, // Jump to a location

    SysCall, // System call
}

impl InstructionArg for OpCode {
    const BIT_SIZE: u32 = 6;

    fn match_to_bytes(data: Self) -> u32 {
        Into::<u8>::into(data) as u32
    }

    fn match_from_bytes(value: u32) -> VMResult<Self> {
        let value = value as u8;
        Self::try_from(value).map_err(|_| VMError::InvalidOpCode(value))
    }
}
