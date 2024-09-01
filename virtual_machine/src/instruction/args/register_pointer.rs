use crate::{error::VMResult, register::Register, sign_extend};

use super::InstructionArg;

/// This is used by move instruction
/// Consist of a register and a literal, wich is used as an offset
///
/// Total size is 12 bits
///
/// ```text
///  11            6 5                     0
/// ┌───────────────┬───────────────────────┐
/// │    REGISTER   |     OFFSET (6 BIT)    │
/// └───────────────┴───────────────────────┘
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterPointer {
    /// 6 Bits
    register: Register,
    /// 6 Bits
    offset: u8,
}

impl InstructionArg for RegisterPointer {
    const BIT_SIZE: u32 = 12;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let register = Register::match_from_bytes(data >> 6)?;
        let offset = sign_extend(data & 0x3f, 6) as u8;

        Ok(Self { register, offset })
    }

    fn match_to_bytes(data: Self) -> u32 {
        let mut result = Register::match_to_bytes(data.register) << 6;
        result |= (data.offset & 0x3f) as u32;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Register;

    #[test]
    fn test_register_pointer() {
        let register_pointer = RegisterPointer {
            register: Register::RA1,
            offset: 0b11111111,
        };

        let bytes = RegisterPointer::match_to_bytes(register_pointer.clone());
        let new_register_pointer = RegisterPointer::match_from_bytes(bytes).unwrap();

        assert_eq!(register_pointer, new_register_pointer);
    }
}
