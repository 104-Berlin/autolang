use std::fmt::Display;

use crate::{error::VMResult, register::Register};

use super::{arg_n::Arg18, InstructionArg};

/// 20 Bit Register or Memory Offset. The first bit is used to determine if the value is a register or a memory offset
///
/// If the first bit is 0, then the value is a register
///
/// If the first bit is 1, then the value is a memory offset
///
/// The Literal is a 8 bit value
///
/// ```text
///  19  18 17                     0
/// ┌────-─┬────────────────────────┐
/// │  00  |      MEMORY_OFFSET     │
/// └────-─┴────────────────────────┘
/// ```
///
/// ```text
///  19  18 17        14 13         0
/// ┌────-─┬────────────┬────────────┐
/// │  10  |  REGISTER  |   UNUSED   │
/// └────-─┴────────────┴────────────┘
/// ```
#[derive(Debug)]
pub enum MemOffsetOrRegister {
    MemOffset(Arg18),
    Register(Register),
}

impl From<Register> for MemOffsetOrRegister {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl<M> From<M> for MemOffsetOrRegister
where
    M: Into<Arg18>,
{
    fn from(mem_offset: M) -> Self {
        Self::MemOffset(mem_offset.into())
    }
}

impl Display for MemOffsetOrRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(register) => write!(f, "{}", register),
            Self::MemOffset(offset) => write!(f, "0x{:X}", offset.0),
        }
    }
}

impl InstructionArg for MemOffsetOrRegister {
    const BIT_SIZE: u32 = 20;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let is_literal = (data >> 9) & 0x1 != 0;
        if is_literal {
            Ok(Self::MemOffset(Arg18::match_from_bytes(data)?))
        } else {
            Ok(Self::Register(Register::match_from_bytes(data)?))
        }
    }
    fn match_to_bytes(data: Self) -> u32 {
        match data {
            Self::Register(register) => Register::match_to_bytes(register) as u32,
            Self::MemOffset(offset) => 0x80000 | (offset.0 & ((1 << Self::BIT_SIZE) - 1)),
        }
    }
}
