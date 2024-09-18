use std::collections::HashMap;

use miette::miette;
use virtual_machine::instruction::{
    args::{jump_cond::JumpCondition, mem_offset::MemOffset},
    Instruction,
};

pub enum Unresolved<T> {
    Unresolved(String),
    Resolved(T),
}

pub enum UnresolvedInstruction {
    Jump {
        cond: JumpCondition,
        offset: Unresolved<MemOffset>,
    },
}

impl UnresolvedInstruction {
    pub fn resolved(
        &self,
        own_addr: u32,
        labels: &HashMap<String, u32>,
    ) -> Result<Instruction, miette::Error> {
        Ok(match self {
            UnresolvedInstruction::Jump { cond, offset } => match offset {
                Unresolved::Resolved(offset) => Instruction::Jump {
                    cond: *cond,
                    dst: (*offset).into(),
                },
                Unresolved::Unresolved(label) => {
                    let label_addr = labels
                        .get(label)
                        .ok_or(miette!("Label {label} not found"))?;

                    let offset = label_addr.wrapping_sub(own_addr);
                    println!("Resolve jump {}", offset as i32);
                    Instruction::Jump {
                        cond: *cond,
                        dst: MemOffset::from(offset).into(),
                    }
                }
            },
        })
    }
}
