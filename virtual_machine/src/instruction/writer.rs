use super::{args::InstructionArg, opcode::OpCode};

pub struct InstructionWriter {
    instruction: u32,
    bit_offset: u32,
}

impl InstructionWriter {
    pub fn new(op_code: OpCode) -> Self {
        Self {
            instruction: 0,
            bit_offset: 0,
        }
        .write::<OpCode>(op_code)
    }

    pub fn write<T: InstructionArg>(mut self, data: T) -> Self {
        T::into_instruction(&mut self.instruction, self.bit_offset, data);
        self.bit_offset += T::BIT_SIZE;
        self
    }

    pub fn finish(self) -> u32 {
        self.instruction
    }
}
