use crate::error::VMResult;

use super::args::InstructionArg;

pub struct InstructionReader {
    instruction: u32,
    bit_offset: u32,
}

impl InstructionReader {
    pub fn new(instruction: u32) -> Self {
        Self {
            instruction,
            bit_offset: 0,
        }
    }

    pub fn read<T: InstructionArg>(&mut self) -> VMResult<T> {
        let result = T::from_instruction(self.instruction, self.bit_offset)?;
        self.bit_offset += T::BIT_SIZE;
        Ok(result)
    }
}
