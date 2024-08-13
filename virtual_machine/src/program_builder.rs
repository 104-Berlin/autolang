use crate::{
    error::VMResult,
    instruction::{Instruction, InstructionPart},
    memory::Memory,
};

pub struct ProgramBuilder<'a> {
    memory: &'a mut dyn Memory,
    addr: u32,
}

impl ProgramBuilder<'_> {
    pub fn new(memory: &mut dyn Memory) -> ProgramBuilder {
        ProgramBuilder { memory, addr: 3000 }
    }

    pub fn add_instruction(mut self, instruction: Instruction) -> VMResult<Self> {
        let instr = Instruction::match_to_bytes(instruction);
        self.memory.write(self.addr, instr)?;

        self.addr += 1;
        Ok(self)
    }
}
