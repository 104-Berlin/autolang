use crate::{error::VMResult, register::Register, sign_extend};

use super::InstructionArg;

/// This is used by move instruction
/// Consist of a register and a literal, wich is used as an offset
///
/// Total size is 20 bits
/// Offset is 14 bits
///
/// ```text
///  19           14 13                     0
/// ┌───────────────┬────────────────────────┐
/// │    REGISTER   |     OFFSET (14 BIT)    │
/// └───────────────┴────────────────────────┘
/// ```
#[derive(Debug, Clone)]
pub struct RegisterPointer {
    register: Register, // 6 Bits
    offset: u16,        // u16?
}

impl InstructionArg for RegisterPointer {
    const BIT_SIZE: u32 = 10;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let register = Register::match_from_bytes(data >> 14)?;
        let offset = sign_extend(data & 0x3FFF, 2) as u16;

        Ok(Self { register, offset })
    }

    fn match_to_bytes(data: Self) -> u32 {
        let mut result = Register::match_to_bytes(data.register) << 14;
        result |= (data.offset & 0x3fff) as u32;

        result
    }
}
