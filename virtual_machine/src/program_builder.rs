use std::collections::HashMap;

use crate::{
    error::{VMError, VMResult},
    instruction::{
        args::{jump_cond::JumpCondition, InstructionArg},
        unresolved_instruction::{Unresolved, UnresolvedInstruction},
        Instruction,
    },
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
    unresolved: Vec<(UnresolvedInstruction, u32)>,
    labels: HashMap<String, u32>,

    blocks: Vec<String>,
}

impl Default for ProgramBuilder {
    fn default() -> Self {
        Self {
            memory: [0; 1024],
            addr: 0,
            unresolved: Vec::new(),
            labels: HashMap::new(),
            blocks: Vec::new(),
        }
    }
}

impl ProgramBuilder {
    pub fn build_instruction(&mut self, instruction: Instruction) -> VMResult<()> {
        let instr = Instruction::match_to_bytes(instruction);
        // (self.memory as <Memory>).write(self.addr, instr);
        // How do i write via the memory trait?
        Memory::write(&mut self.memory, self.addr, instr)?;

        self.addr += 1;
        Ok(())
    }

    pub fn build_instruction_unresolved(
        &mut self,
        instruction: UnresolvedInstruction,
    ) -> VMResult<()> {
        // We try resolving them right now.
        match instruction.resolved(self.addr, &self.labels) {
            Ok(instruction) => self.build_instruction(instruction),
            Err(_) => {
                self.unresolved.push((instruction, self.addr));
                self.addr += 1;
                Ok(())
            }
        }
    }

    pub fn build_unconditional_jump(&mut self, block: usize) -> VMResult<()> {
        let label = self.get_block_label(block)?;

        self.build_instruction_unresolved(UnresolvedInstruction::Jump {
            cond: JumpCondition::Always,
            offset: Unresolved::Unresolved(label),
        })
    }

    pub fn build_conditional_jump(&mut self, block: usize, cond: JumpCondition) -> VMResult<()> {
        let label = self.get_block_label(block)?;

        self.build_instruction_unresolved(UnresolvedInstruction::Jump {
            cond,
            offset: Unresolved::Unresolved(label),
        })
    }

    pub fn add_value(&mut self, addr: u32, value: u32) -> VMResult<()> {
        self.memory.write(addr, value)?;

        Ok(())
    }

    pub fn append_block(&mut self, label: Option<&'static str>) -> usize {
        let block_id = self.blocks.len() + 1;
        let label = format!("{}__block{}", label.unwrap_or(""), block_id);
        self.blocks.push(label);
        block_id
    }

    pub fn block_insertion_point(&mut self, block: usize) -> VMResult<()> {
        let block_label = self.get_block_label(block)?;
        if self.labels.contains_key(&block_label) {
            return Err(VMError::BlockAlreadyDefined(block));
        }

        self.labels.insert(block_label, self.addr);

        Ok(())
    }

    pub fn finish(mut self) -> VMResult<[u32; 1024]> {
        self.resolve_instructions()?;
        Ok(self.memory)
    }

    fn resolve_instructions(&mut self) -> VMResult<()> {
        for instr in self.unresolved.iter() {
            let instruction = instr.0.resolved(instr.1, &self.labels)?;

            self.memory
                .write(instr.1, Instruction::match_to_bytes(instruction))?;
        }

        Ok(())
    }

    fn get_block_label(&self, block: usize) -> VMResult<String> {
        self.blocks
            .get(block - 1)
            .cloned()
            .ok_or(VMError::BlockNotFound(block))
    }
}
