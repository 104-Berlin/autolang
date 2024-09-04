use std::collections::HashMap;

use miette::{miette, Context, IntoDiagnostic, LabeledSpan, SourceSpan};
use virtual_machine::{
    instruction::{
        args::{jump_cond::JumpCondition, logical_operator::LogicalOperator, InstructionArg},
        Instruction,
    },
    memory::Memory,
    register::Register,
};

use crate::{
    prelude::{Spanned, TypeID},
    spanned::{SpanExt, WithSpan},
    ALResult,
};

use super::{
    scope::Scope,
    unresolved_instruction::{Unresolved, UnresolvedInstruction},
};

pub type Block = usize;

pub trait Buildable {
    fn build(&self, builder: &mut CompilerContext) -> ALResult<()>;
}

pub enum VarLocation {
    Local(u32),  // Offset from the base pointer
    Global(u32), // Memory address
}

#[derive(Default)]
pub struct SymbolTable {
    globals: HashMap<String, (u32, TypeID)>, // Stored in the memory of the programm. Probably need sections then
    scopes: Option<Scope>,                   // Can we make this non optional?
}

pub struct CompilerContext {
    /// For now 4kb programs? aka 1024 instructions and static values.
    /// Maybe we need some kind of resizable memory?
    memory: [u32; 1024],
    addr: u32,
    unresolved: Vec<(Spanned<UnresolvedInstruction>, u32)>,

    labels: HashMap<String, u32>,

    // Some not yet defined labels
    blocks: Vec<String>,
    symbol_table: SymbolTable,

    current_continue_block: Option<Block>,
    current_break_block: Option<Block>,
}

impl Default for CompilerContext {
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

impl CompilerContext {
    pub fn build_instruction(&mut self, instruction: Spanned<Instruction>) -> ALResult<()> {
        let instr = Instruction::match_to_bytes(instruction.value);
        // (self.memory as <Memory>).write(self.addr, instr);
        // How do i write via the memory trait?
        Memory::write(&mut self.memory, self.addr, instr).into_diagnostic()?;

        self.addr += 4;
        Ok(().with_span(instruction.span))
    }

    pub fn build_instruction_unresolved(
        &mut self,
        instruction: Spanned<UnresolvedInstruction>,
    ) -> ALResult<()> {
        // We try resolving them right now.
        match instruction.resolved(self.addr, &self.labels) {
            Ok(resolved) => self.build_instruction(resolved.with_span(instruction.span)),
            Err(_) => {
                let span = instruction.span;
                self.unresolved.push((instruction, self.addr));
                self.addr += 4;
                Ok(().with_span(span))
            }
        }
    }

    pub fn build_unconditional_jump(&mut self, block: Block, span: SourceSpan) -> ALResult<()> {
        let label = self.get_block_label(block)?;

        self.build_instruction_unresolved(
            UnresolvedInstruction::Jump {
                cond: JumpCondition::Always,
                offset: Unresolved::Unresolved(label),
            }
            .with_span(span),
        )
    }

    pub fn build_conditional_jump(
        &mut self,
        block: Block,
        cond: JumpCondition,
        span: SourceSpan,
    ) -> ALResult<()> {
        let label = self.get_block_label(block)?;

        self.build_instruction_unresolved(
            UnresolvedInstruction::Jump {
                cond,
                offset: Unresolved::Unresolved(label),
            }
            .with_span(span),
        )
    }

    // Expects the value for the var to be in RA1
    pub fn build_local_var(&mut self, sym: &Spanned<String>, typ: TypeID) -> ALResult<()> {
        self.build_instruction(Instruction::Push(Register::RA1.into()).with_span(sym.span))?;
        // Push the var to the symbol table
        self.symbol_table
            .scopes
            .as_mut()
            .ok_or(miette!(
                labels = vec![LabeledSpan::at(sym.span, ""),],
                "You are in the top level scope. You can't define a local variable here."
            ))?
            .push_variable((**sym).clone(), typ);

        Ok(().with_span(sym.span))
    }

    /// Requires RA2 = lhs and RA1 = rhs
    pub fn build_compare(&mut self, operator: Spanned<LogicalOperator>) -> ALResult<()> {
        self.build_instruction(
            Instruction::Compare {
                lhs: Register::RA2.into(),
                rhs: Register::RA1.into(),
            }
            .with_span(operator.span),
        )
        .wrap_err("Building Compare")?;
        // Load the result of the comparison into RA1
        self.build_instruction(
            Instruction::LoadBool {
                dst: Register::RA1,
                op: operator.value,
            }
            .with_span(operator.span),
        )
        .wrap_err("Building Compare")?;

        Ok(().with_span(operator.span))
    }

    pub fn find_var(&self, sym: &Spanned<String>) -> Option<(VarLocation, TypeID)> {
        if let Some((offset, typ)) = self.symbol_table.scopes.as_ref()?.get(sym) {
            return Some((VarLocation::Local(offset), typ));
        }

        if let Some((addr, typ)) = self.symbol_table.globals.get(&**sym) {
            return Some((VarLocation::Global(*addr), typ.clone()));
        }

        None
    }

    pub fn append_block(&mut self, label: Option<&'static str>) -> Block {
        let block_id = self.blocks.len() + 1;
        let label = format!("{}__block{}", label.unwrap_or(""), block_id);
        self.blocks.push(label);
        block_id
    }

    pub fn block_insertion_point(&mut self, block: Block, span: SourceSpan) -> ALResult<()> {
        let block_label = self.get_block_label(block)?;
        if self.labels.contains_key(&block_label) {
            return Err(miette!("Block already defined"));
        }

        self.labels.insert(block_label, self.addr);

        Ok(().with_span(span))
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
        let total_span = self.resolve_instructions()?.span;
        Ok(self.memory.with_span(total_span))
    }

    fn resolve_instructions(&mut self) -> ALResult<()> {
        let mut span: Option<SourceSpan> = None;
        for instr in self.unresolved.iter() {
            let instruction = instr.0.resolved(instr.1, &self.labels)?;

            self.memory
                .write(instr.1, Instruction::match_to_bytes(instruction))
                .into_diagnostic()?;

            match span {
                Some(ref mut s) => *s = s.union(&instr.0.span),
                None => span = Some(instr.0.span),
            }
        }

        Ok(().with_span(span.unwrap_or(SourceSpan::from(0..0))))
    }

    fn get_block_label(&self, block: Block) -> Result<String, miette::Error> {
        self.blocks
            .get(block - 1)
            .cloned()
            .ok_or(miette!("Block ({}) not found", block))
    }
}
