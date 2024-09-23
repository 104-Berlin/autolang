//! This is either an register or a register pointer
//!
//! Register pointers read the address from the register and then add the offset.
//! Used when you want to read from a specific address in memory.
//! Mostly used by stack operations for functions calls and local variables.

use std::fmt::Display;

use crate::{error::VMResult, machine::Machine, register::Register};

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

impl Display for RegisterOrRegisterPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(register) => write!(f, "{}", register),
            Self::RegisterPointer(register_pointer) => write!(f, "{}", register_pointer),
        }
    }
}

impl RegisterOrRegisterPointer {
    pub fn read(&self, machine: &Machine) -> VMResult<u32> {
        match self {
            Self::Register(register) => Ok(machine.registers().get(*register)),
            Self::RegisterPointer(register_pointer) => register_pointer.read(machine),
        }
    }

    pub fn write(&self, machine: &mut Machine, value: u32) -> VMResult<()> {
        match self {
            Self::Register(register) => {
                machine.registers_mut().set(*register, value);
                Ok(())
            }
            Self::RegisterPointer(register_pointer) => register_pointer.write(machine, value),
        }
    }
}

impl From<Register> for RegisterOrRegisterPointer {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl From<RegisterPointer> for RegisterOrRegisterPointer {
    fn from(register_pointer: RegisterPointer) -> Self {
        Self::RegisterPointer(register_pointer)
    }
}

//          |           ARG1        |           ARG2           |
//          |                       |                          |
//  MOVE      PTR  REGISTER  OFFSET      PTR  REGISTER  OFFSET
// 000110      0    000000   000000       1    001011   000000

impl InstructionArg for RegisterOrRegisterPointer {
    const BIT_SIZE: u32 = 13;

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
