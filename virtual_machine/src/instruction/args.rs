use crate::error::{VMError, VMResult};

pub mod arg20;
pub mod jump_cond;
pub mod logical_operator;
pub mod mem_offset;
pub mod register_or_literal;
pub mod register_pointer;
pub mod unused;

pub trait InstructionArg {
    const BIT_SIZE: u32;

    /// Match the instruction from the bytes
    /// The bit_offset is used to skip the bits that are not part of the instruction
    /// We are cutting all the bits that are not part of the instruction and put the instruction in the first bits
    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized;

    /// Match the instruction to the bytes
    /// This is used to convert the instruction back to the bytes
    /// The data should be on the most right side with BIT_SIZE bits
    fn match_to_bytes(data: Self) -> u32;

    fn from_instruction(instruction: u32, bit_offset: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        if bit_offset + Self::BIT_SIZE > 32 {
            return Err(VMError::FailedParsingInstruction(instruction));
        }
        let offset = 32 - (Self::BIT_SIZE + bit_offset);
        let code = (instruction >> offset) & ((1 << Self::BIT_SIZE) - 1);

        Self::match_from_bytes(code)
    }

    fn into_instruction(instruction: &mut u32, bit_offset: u32, data: Self)
    where
        Self: Sized,
    {
        *instruction |= Self::match_to_bytes(data) << (32 - (Self::BIT_SIZE + bit_offset));
    }
}

impl InstructionArg for bool {
    const BIT_SIZE: u32 = 1;

    fn match_to_bytes(data: Self) -> u32 {
        data as u32
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(data & 0x1 != 0)
    }
}

impl InstructionArg for u8 {
    const BIT_SIZE: u32 = 8;

    fn match_to_bytes(data: Self) -> u32 {
        data as u32
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(data as u8)
    }
}
