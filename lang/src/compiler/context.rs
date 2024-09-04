use std::collections::HashMap;

use miette::{miette, LabeledSpan};
use virtual_machine::{
    instruction::{args::jump_cond::JumpCondition, Instruction},
    memory::Memory,
    register::Register,
};

use crate::{prelude::Spanned, ALResult};

use super::{
    scope::Scope,
    unresolved_instruction::{Unresolved, UnresolvedInstruction},
};

pub type Block = usize;

pub trait Buildable {
    type Error;

    fn build(&self, builder: &mut Context) -> Result<(), Self::Error>;
}

pub enum VarLocation {
    Local(u32),  // Offset from the base pointer
    Global(u32), // Memory address
}

#[derive(Default)]
pub struct SymbolTable {
    globals: HashMap<String, u32>, // Stored in the memory of the programm. Probably need sections then
    scopes: Option<Scope>,         // Can we make this non optional?
}

pub struct Context {
    /// For now 4kb programs? aka 1024 instructions and static values.
    /// Maybe we need some kind of resizable memory?
    memory: [u32; 1024],
    addr: u32,
    unresolved: Vec<(UnresolvedInstruction, u32)>,

    labels: HashMap<String, u32>,

    // Some not yet defined labels
    blocks: Vec<String>,
    symbol_table: SymbolTable,

    current_continue_block: Option<Block>,
    current_break_block: Option<Block>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            memory: [0; 1024],
            addr: 0,
            unresolved: Vec::new(),
            labels: HashMap::new(),
            blocks: Vec::new(),
            symbol_table: SymbolTable::default(),
            current_continue_block: None,
            current_break_block: None,
        }
    }
}

impl Context {
    pub fn build_instruction(&mut self, instruction: Instruction) -> ALResult<()> {
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
    ) -> ALResult<()> {
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

    pub fn build_unconditional_jump(&mut self, block: Block) -> ALResult<()> {
        let label = self.get_block_label(block)?;

        self.build_instruction_unresolved(UnresolvedInstruction::Jump {
            cond: JumpCondition::Always,
            offset: Unresolved::Unresolved(label),
        })
    }

    pub fn build_conditional_jump(&mut self, block: Block, cond: JumpCondition) -> ALResult<()> {
        let label = self.get_block_label(block)?;

        self.build_instruction_unresolved(UnresolvedInstruction::Jump {
            cond,
            offset: Unresolved::Unresolved(label),
        })
    }

    // Expects the value for the var to be in RA1
    pub fn build_local_var(&mut self, sym: Spanned<String>) -> ALResult<()> {
        self.build_instruction(Instruction::Push(Register::RA1.into()))?;
        // Push the var to the symbol table
        self.symbol_table
            .scopes
            .as_mut()
            .ok_or(miette!(
                labels = vec![LabeledSpan::at(sym.span, ""),],
                "You are in the top level scope. You can't define a local variable here."
            ))?
            .push_variable(sym);

        Ok(())
    }

    // This should be renamed to add global and use the symbol table
    pub fn add_value(&mut self, addr: u32, value: u32) -> ALResult<()> {
        self.memory.write(addr, value)?;

        Ok(())
    }

    pub fn find_var(&self, sym: &str) -> Option<VarLocation> {
        if let Some(offset) = self.symbol_table.scopes.as_ref()?.get(sym) {
            return Some(VarLocation::Local(offset));
        }

        if let Some(addr) = self.symbol_table.globals.get(sym) {
            return Some(VarLocation::Global(*addr));
        }

        None
    }

    pub fn append_block(&mut self, label: Option<&'static str>) -> Block {
        let block_id = self.blocks.len() + 1;
        let label = format!("{}__block{}", label.unwrap_or(""), block_id);
        self.blocks.push(label);
        block_id
    }

    pub fn block_insertion_point(&mut self, block: Block) -> ALResult<()> {
        let block_label = self.get_block_label(block)?;
        if self.labels.contains_key(&block_label) {
            return Err(miette!("Block already defined"));
        }

        self.labels.insert(block_label, self.addr);

        Ok(())
    }

    pub fn push_scope(&mut self) {
        match self.symbol_table.scopes.take() {
            Some(scope) => {
                self.symbol_table.scopes = Some(scope.new_child());
            }
            None => self.symbol_table.scopes = Some(Scope::new()),
        }
    }

    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.symbol_table.scopes.take() {
            self.symbol_table.scopes = scope.pop_child(); // Sets root to None if we are at the root
        }
    }

    pub fn set_continue_block(&mut self, block: Block) {
        self.current_continue_block = Some(block);
    }

    pub fn pop_continuer_block(&mut self) {
        self.current_continue_block = None;
    }

    pub fn get_continue_block(&self) -> Option<Block> {
        self.current_continue_block
    }

    pub fn set_break_block(&mut self, block: Block) {
        self.current_break_block = Some(block);
    }

    pub fn pop_break_block(&mut self) {
        self.current_break_block = None;
    }

    pub fn get_break_block(&self) -> Option<Block> {
        self.current_break_block
    }

    pub fn finish(mut self) -> ALResult<[u32; 1024]> {
        self.resolve_instructions()?;
        Ok(self.memory)
    }

    fn resolve_instructions(&mut self) -> ALResult<()> {
        for instr in self.unresolved.iter() {
            let instruction = instr.0.resolved(instr.1, &self.labels)?;

            self.memory
                .write(instr.1, Instruction::match_to_bytes(instruction))?;
        }

        Ok(())
    }

    fn get_block_label(&self, block: Block) -> ALResult<String> {
        self.blocks
            .get(block - 1)
            .cloned()
            .ok_or(miette!("Block ({block}) not found"))
    }
}
