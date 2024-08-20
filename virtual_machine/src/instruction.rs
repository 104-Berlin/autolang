use std::fmt::Display;

use args::{
    arg20::Arg20, jump_cond::JumpCondition, register_or_literal::RegisterOrLiteral, InstructionArg,
};
use reader::InstructionReader;
use writer::InstructionWriter;

use crate::{error::VMResult, register::Register};
use opcode::OpCode;

pub mod args;
pub mod opcode;
pub mod reader;
pub mod writer;

#[derive(Debug)]
pub enum Instruction {
    Halt,
    Nop,
    Load {
        dst: Register,
        addr: Arg20,
    },
    Imm {
        dst: Register,
        value: Arg20,
    },
    Add {
        dst: Register,
        lhs: RegisterOrLiteral,
        rhs: RegisterOrLiteral,
    },
    Compare {
        lhs: RegisterOrLiteral,
        rhs: RegisterOrLiteral,
    },
    Jump {
        cond: JumpCondition,
        offset: Arg20,
    },

    Push(Register),
    Pop(Register),
}

impl InstructionArg for Instruction {
    const BIT_SIZE: u32 = 32;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let mut reader = InstructionReader::new(data);

        let op_code: OpCode = reader.read()?;

        match op_code {
            OpCode::Halt => Ok(Self::Halt),
            OpCode::Nop => Ok(Self::Nop),
            OpCode::Load => Ok(Self::Load {
                dst: reader.read()?,
                addr: reader.read()?,
            }),
            OpCode::Imm => Ok(Self::Imm {
                dst: reader.read()?,
                value: reader.read()?,
            }),
            OpCode::Add => Ok(Self::Add {
                dst: reader.read()?,
                lhs: reader.read()?,
                rhs: reader.read()?,
            }),
            OpCode::Jump => Ok(Self::Jump {
                cond: reader.read()?,
                offset: reader.read()?,
            }),
            OpCode::Compare => Ok(Instruction::Compare {
                lhs: reader.read()?,
                rhs: reader.read()?,
            }),
            OpCode::Push => Ok(Instruction::Push(reader.read()?)),
            OpCode::Pop => Ok(Instruction::Pop(reader.read()?)),
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        let mut writer = InstructionWriter::new(match data {
            Self::Halt => OpCode::Halt,
            Self::Nop => OpCode::Nop,
            Self::Load { .. } => OpCode::Load,
            Self::Imm { .. } => OpCode::Imm,
            Self::Add { .. } => OpCode::Add,
            Self::Jump { .. } => OpCode::Jump,
            Self::Compare { .. } => OpCode::Compare,
            Self::Push(_) => OpCode::Push,
            Self::Pop(_) => OpCode::Pop,
        });

        match data {
            Self::Halt => (),
            Self::Nop => (),
            Self::Load { dst, addr } => {
                writer = writer.write(dst).write(addr);
            }
            Self::Imm { dst, value } => {
                writer = writer.write(dst).write(value);
            }
            Self::Add { dst, lhs, rhs } => {
                writer = writer.write(dst).write(lhs).write(rhs);
            }
            Self::Jump { cond, offset } => {
                // We have some unused bits here
                // We need to write them, in order to skip to the correct bit for the offset
                writer = writer.write(cond).write(offset);
            }
            Self::Compare { lhs, rhs } => {
                writer = writer.write(lhs).write(rhs);
            }
            Self::Push(reg) => {
                writer = writer.write(reg);
            }
            Self::Pop(reg) => {
                writer = writer.write(reg);
            }
        }

        writer.finish()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Halt => write!(f, "Halt"),
            Self::Nop => write!(f, "Nop"),
            Self::Load { dst, addr } => write!(f, "Load {}, {}", dst, addr.0),
            Self::Imm { dst, value } => write!(f, "Imm {}, {}", dst, value.0),
            Self::Add { dst, lhs, rhs } => write!(f, "Add {}, {}, {}", dst, lhs, rhs),
            Self::Jump { cond, offset } => write!(f, "Jump {:?} {}", cond, offset.0 as i32),
            Self::Compare { lhs, rhs } => write!(f, "Compare {}, {}", lhs, rhs),
            Self::Push(reg) => write!(f, "Push {}", reg),
            Self::Pop(reg) => write!(f, "Pop {}", reg),
        }
    }
}
