//! This is either an register or a register pointer
//!
//! Register pointers read the address from the register and then add the offset.
//! Used when you want to read from a specific address in memory.
//! Mostly used by stack operations for functions calls and local variables.

use crate::{error::VMResult, register::Register};

use super::{register_pointer::RegisterPointer, InstructionArg};

/// This is used by move instruction
/// Consist of a register and a literal, wich is used as an offset
///
/// Total size is 13 bits
///
/// ```text
///    12    11        6 5                     0
/// ┌───────┬───────────┬───────────────────────┐
/// │ PTR?  |  REGISTER |     OFFSET (6 BIT)    │
/// └───────┴───────────┴───────────────────────┘
///            |  REGISTER OR POINTER   |
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterOrRegisterPointer {
    Register(Register),
    RegisterPointer(RegisterPointer),
}

impl InstructionArg for RegisterOrRegisterPointer {
    const BIT_SIZE: u32 = 12;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let is_pointer = (data >> 12) & 0x1 == 1;

        match is_pointer {
            true => {
                let register_pointer = RegisterPointer::match_from_bytes(data)?;
                Ok(Self::RegisterPointer(register_pointer))
            }
            false => {
                let register = Register::match_from_bytes(data >> 6)?;
                Ok(Self::Register(register))
            }
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        match data {
            Self::Register(register) => Register::match_to_bytes(register) << 6,
            Self::RegisterPointer(register_pointer) => {
                RegisterPointer::match_to_bytes(register_pointer) | (1 << 12)
            }
        }
    }
}
