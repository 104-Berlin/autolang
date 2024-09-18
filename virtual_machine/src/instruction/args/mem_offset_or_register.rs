use std::fmt::Display;

use crate::{error::VMResult, machine::Machine, register::Register, sign_extend};

use super::{mem_offset::MemOffset, InstructionArg};

/// 22 Bit Register or Memory Offset. The first bit is used to determine if the value is a register or a memory offset
///
/// If the first bit is 0, then the value is a register
///
/// If the first bit is 1, then the value is a memory offset
///
/// The Literal is a 8 bit value
///
/// ```text
///  21  20 19                     0
/// ┌────-─┬────────────────────────┐
/// │  0   |      MEMORY_OFFSET     │
/// └────-─┴────────────────────────┘
/// ```
///
/// ```text
///  21  20 19        14 13         0
/// ┌────-─┬────────────┬────────────┐
/// │  1   |  REGISTER  |   UNUSED   │
/// └────-─┴────────────┴────────────┘
/// ```
#[derive(Debug)]
pub enum MemOffsetOrRegister {
    MemOffset(MemOffset),
    Register(Register),
}

impl From<Register> for MemOffsetOrRegister {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl From<MemOffset> for MemOffsetOrRegister {
    fn from(mem_offset: MemOffset) -> Self {
        Self::MemOffset(mem_offset)
    }
}

impl Display for MemOffsetOrRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(register) => write!(f, "{}", register),
            Self::MemOffset(offset) => write!(f, "0x{:X}", offset.offset),
        }
    }
}

impl InstructionArg for MemOffsetOrRegister {
    const BIT_SIZE: u32 = 10;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let is_literal = (data >> 9) & 0x1 != 0;
        if is_literal {
            Ok(Self::MemOffset(MemOffset::match_from_bytes(data)?))
        } else {
            Ok(Self::Register(Register::match_from_bytes(data)?))
        }
    }
    fn match_to_bytes(data: Self) -> u32 {
        match data {
            Self::Register(register) => register as u32,
            Self::MemOffset(offset) => 0x100000 | (offset.offset as u32),
        }
    }
}
