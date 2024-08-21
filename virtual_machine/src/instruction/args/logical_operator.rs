use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::InstructionArg;

/// When editing update the execution of the jump instruction
/// in the `Machine` struct
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum LogicalOperator {
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
}

impl InstructionArg for LogicalOperator {
    const BIT_SIZE: u32 = 6;

    fn match_from_bytes(data: u32) -> crate::error::VMResult<Self>
    where
        Self: Sized,
    {
        Self::try_from(data as u8)
            .map_err(|_| crate::error::VMError::FailedParsingInstruction(data))
    }

    fn match_to_bytes(data: Self) -> u32 {
        Into::<u8>::into(data) as u32
    }
}
