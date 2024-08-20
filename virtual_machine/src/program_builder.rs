use crate::{
    error::VMResult,
    instruction::{args::InstructionArg, Instruction},
    memory::Memory,
};

pub trait Buildable {
    type Error;

    fn build(&self, builder: &mut ProgramBuilder) -> Result<(), Self::Error>;
}

pub struct ProgramBuilder {
    /// For now 4kb programs? aka 1024 instructions and static values.
    /// Maybe we need some kind of resizable memory?
    memory: [u32; 1024],
    addr: u32,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            memory: [0; 1024],
            addr: 0,
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> VMResult<()> {
        let instr = Instruction::match_to_bytes(instruction);
        // (self.memory as <Memory>).write(self.addr, instr);
        // How do i write via the memory trait?
        Memory::write(&mut self.memory, self.addr, instr)?;

        self.addr += 1;
        Ok(())
    }

    pub fn add_value(&mut self, addr: u32, value: u32) -> VMResult<()> {
        self.memory.write(addr, value)?;

        Ok(())
    }

    pub fn finish(self) -> [u32; 1024] {
        self.memory
    }
}
