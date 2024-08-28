use std::ops::Deref;

use crate::error::VMResult;

use super::InstructionArg;

#[derive(Debug, Clone, Copy)]
pub struct MemOffset {
    pub offset: u32,
}

impl InstructionArg for MemOffset {
    const BIT_SIZE: u32 = 20;

    fn match_to_bytes(data: Self) -> u32 {
        data.offset & 0xF_FFFF
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(Self {
            offset: data & 0xF_FFFF,
        })
    }
}

impl From<u32> for MemOffset {
    fn from(offset: u32) -> Self {
        Self { offset }
    }
}

impl Deref for MemOffset {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.offset
    }
}
