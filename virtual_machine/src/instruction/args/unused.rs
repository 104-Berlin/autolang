use crate::error::VMResult;

use super::InstructionArg;

pub struct Unused<const T: u32>;

impl<const T: u32> InstructionArg for Unused<T> {
    const BIT_SIZE: u32 = T;

    fn match_from_bytes(_: u32) -> VMResult<Self> {
        Ok(Self)
    }

    fn match_to_bytes(_: Self) -> u32 {
        0
    }
}
