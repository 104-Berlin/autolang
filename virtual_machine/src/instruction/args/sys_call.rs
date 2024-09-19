use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::InstructionArg;

#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SysCall {
    SysFunc,
}

impl InstructionArg for SysCall {
    const BIT_SIZE: u32 = 6;

    fn match_from_bytes(data: u32) -> crate::error::VMResult<Self>
    where
        Self: Sized,
    {
        Self::try_from((data & Self::MASK) as u8)
            .map_err(|_| crate::error::VMError::FailedParsingInstruction(data))
    }

    fn match_to_bytes(data: Self) -> u32 {
        (Into::<u8>::into(data) as u32) & Self::MASK
    }
}
