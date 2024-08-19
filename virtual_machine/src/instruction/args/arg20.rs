use crate::error::VMResult;

use super::InstructionArg;

#[derive(Debug)]
pub struct Arg20(pub u32);

impl InstructionArg for Arg20 {
    const BIT_SIZE: u32 = 20;

    fn match_to_bytes(data: Self) -> u32 {
        data.0 & 0xF_FFFF
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(Self(data & 0xF_FFFF))
    }
}
