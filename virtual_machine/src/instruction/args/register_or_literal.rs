use std::fmt::Display;

use crate::{error::VMResult, machine::Machine, register::Register, sign_extend};

use super::InstructionArg;

/// 10 Bit Register or Literal. The first bit is used to determine if the value is a register or a literal
///
/// If the first bit is 0, then the value is a register
///
/// If the first bit is 1, then the value is a literal
///
/// The Literal is a 8 bit value
///
/// ```text
///    9   8          6 5          0
/// ┌─────┬────────────┬────────────┐
/// │  0  |   UNUSED   |    REGA    │
/// └─────┴────────────┴────────────┘
/// ```
///
/// ```text
///    9         8      7          0
/// ┌─────┬────────────┬────────────┐
/// │  1  |   UNUSED   |  LITERAL8  │
/// └─────┴────────────┴────────────┘
/// ```
#[derive(Debug)]
pub enum RegisterOrLiteral {
    Register(Register),
    Literal(u8),
}

impl RegisterOrLiteral {
    /// Get the value of the register or the literal itself
    pub fn get_val(&self, machine: &Machine) -> u32 {
        match self {
            Self::Register(register) => machine.registers().get(*register),
            Self::Literal(literal) => sign_extend(*literal as u32, 8),
        }
    }
}

impl From<Register> for RegisterOrLiteral {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl Display for RegisterOrLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(register) => write!(f, "{}", register),
            Self::Literal(literal) => write!(f, "0x{:X}", literal),
        }
    }
}

impl InstructionArg for RegisterOrLiteral {
    const BIT_SIZE: u32 = 10;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let is_literal = (data >> 9) & 0x1 != 0;
        if is_literal {
            Ok(Self::Literal((data & 0xFF) as u8))
        } else {
            Ok(Self::Register(Register::match_from_bytes(data)?))
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        match data {
            Self::Register(register) => register as u32,
            Self::Literal(literal) => 0x200 | (literal as u32),
        }
    }
}
