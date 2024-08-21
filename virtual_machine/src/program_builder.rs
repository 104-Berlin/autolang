use std::collections::HashMap;

use crate::{
    error::{VMError, VMResult},
    instruction::{
        args::{arg20::Arg20, InstructionArg},
        Instruction,
    },
    memory::Memory,
};

pub struct UnresolvedInstruction {
    instruction: Instruction,
    label: String,
    addr: u32,
}

pub trait Buildable {
    type Error;

    fn build(&self, builder: &mut ProgramBuilder) -> Result<(), Self::Error>;
}

pub struct ProgramBuilder {
    /// For now 4kb programs? aka 1024 instructions and static values.
    /// Maybe we need some kind of resizable memory?
    memory: [u32; 1024],
    addr: u32,
    unresolved: Vec<UnresolvedInstruction>,
    labels: HashMap<String, u32>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            memory: [0; 1024],
            addr: 0,
            unresolved: Vec::new(),
            labels: HashMap::new(),
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

    pub fn add_unresolved(&mut self, instruction: Instruction, label: impl Into<String>) {
        self.unresolved.push(UnresolvedInstruction {
            instruction,
            label: label.into(),
            addr: self.addr,
        });
        self.addr += 1;
    }

    pub fn add_value(&mut self, addr: u32, value: u32) -> VMResult<()> {
        self.memory.write(addr, value)?;

        Ok(())
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.insert(label, self.addr);
    }

    pub fn finish(mut self) -> VMResult<[u32; 1024]> {
        self.resolve_instructions()?;
        Ok(self.memory)
    }

    fn resolve_instructions(&mut self) -> VMResult<()> {
        for instr in self.unresolved.iter() {
            let addr = self
                .labels
                .get(&instr.label)
                .ok_or(VMError::LabelNotFound("Label not found".into()))?;

            let offset = *addr as i32 - instr.addr as i32;

            match instr.instruction {
                Instruction::Jump { cond, .. } => self.memory.write(
                    instr.addr,
                    Instruction::match_to_bytes(Instruction::Jump {
                        cond,
                        offset: Arg20(offset as u32),
                    }),
                )?,
                Instruction::Load { dst, .. } => self.memory.write(
                    instr.addr,
                    Instruction::match_to_bytes(Instruction::Load {
                        dst,
                        offset: Arg20(offset as u32),
                    }),
                )?,
                _ => {}
            };
        }

        Ok(())
    }
}
