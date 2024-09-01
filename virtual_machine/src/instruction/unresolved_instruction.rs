use std::collections::HashMap;

use crate::{
    error::{VMError, VMResult},
    register::Register,
};

use super::{
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
    pub fn resolved(&self, own_addr: u32, labels: &HashMap<String, u32>) -> VMResult<Instruction> {
        Ok(match self {
            UnresolvedInstruction::Jump { cond, offset } => match offset {
                Unresolved::Resolved(offset) => Instruction::Jump {
                    cond: *cond,
                    offset: *offset,
                },
                Unresolved::Unresolved(label) => {
                    let label_addr = labels
                        .get(label)
                        .ok_or(VMError::LabelNotFound(label.clone()))?;

                    let offset = label_addr.wrapping_sub(own_addr);
                    Instruction::Jump {
                        cond: *cond,
                        offset: MemOffset::from(offset),
                    }
                }
            },
        })
    }
}
