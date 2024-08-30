use crate::{
    error::{VMError, VMResult},
    instruction::InstructionPart,
};

/// # 6 Bit
#[derive(Debug)]
pub enum OpCode {
    Halt,
    Nop,
    Load,
}

impl InstructionPart for OpCode {
    type Output = Self;
    const BIT_SIZE: u32 = 6;

    fn match_to_bytes(data: Self::Output) -> u32 {
        data as u32
    }

    fn match_from_bytes(value: u32) -> VMResult<Self> {
        let value = value as u8;

        match value {
            0x0 => Ok(OpCode::Halt),
            0x1 => Ok(OpCode::Nop),
            0x2 => Ok(OpCode::Load),
            _ => Err(VMError::InvalidOpCode(value)),
        }
    }
}
