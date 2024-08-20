use crate::{
    error::VMResult,
    instruction::{args::InstructionArg, Instruction},
    machine::Machine,
    memory::Memory,
};

pub struct ProgramBuilder {
    memory: Box<dyn Memory>,
    addr: u32,
}

impl ProgramBuilder {
    pub fn new(memory: impl Memory + 'static) -> ProgramBuilder {
        ProgramBuilder {
            memory: Box::new(memory),
            addr: 3000,
        }
    }

    pub fn add_instruction(mut self, instruction: Instruction) -> VMResult<Self> {
        let instr = Instruction::match_to_bytes(instruction);
        self.memory.write(self.addr, instr)?;

        self.addr += 1;
        Ok(self)
    }

    pub fn add_value(mut self, addr: u32, value: u32) -> VMResult<Self> {
        self.memory.write(addr, value)?;

        Ok(self)
    }

    pub fn finish(self) -> Machine {
        Machine::new(self.memory)
    }
}
