use std::fmt::Display;

use args::{
    arg_n::Arg20, jump_cond::JumpCondition, logical_operator::LogicalOperator,
    mem_offset_or_register::MemOffsetOrRegister, register_or_literal::RegisterOrLiteral,
    register_or_register_pointer::RegisterOrRegisterPointer, sys_call::SysCall, InstructionArg,
};
use reader::InstructionReader;
use writer::InstructionWriter;

use crate::{error::VMResult, register::Register};
use opcode::OpCode;

pub mod args;
pub mod opcode;
pub mod reader;
pub mod writer;

/// ```text
///  31      26 25         20 19     0
/// ┌──────────┬─────────────┬────────┐
/// │  OPCODE  | REGISTER-A? |  REST  │
/// └──────────┴─────────────┴────────┘
/// ```

#[derive(Debug)]
pub enum Instruction {
    Halt,
    Nop,
    // This loads a bool from the condition register
    // This seems to be a bit of waste, but wanted to get it in for now
    // Maybe it works fine
    LoadBool {
        dst: Register,
        op: LogicalOperator,
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
        dst: MemOffsetOrRegister,
    },
    Move {
        dst: RegisterOrRegisterPointer,
        src: RegisterOrRegisterPointer,
    },

    Push(RegisterOrLiteral),
    Pop(Register),

    SysCall(SysCall),
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
            OpCode::LoadBool => Ok(Self::LoadBool {
                dst: reader.read()?,
                op: reader.read()?,
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
                dst: reader.read()?,
            }),

            OpCode::Compare => Ok(Instruction::Compare {
                lhs: reader.read()?,
                rhs: reader.read()?,
            }),
            OpCode::Move => Ok(Instruction::Move {
                dst: reader.read()?,
                src: reader.read()?,
            }),
            OpCode::Push => Ok(Instruction::Push(reader.read()?)),
            OpCode::Pop => Ok(Instruction::Pop(reader.read()?)),
            OpCode::SysCall => Ok(Instruction::SysCall(reader.read()?)),
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        let mut writer = InstructionWriter::new(match data {
            Self::Halt => OpCode::Halt,
            Self::Nop => OpCode::Nop,
            Self::LoadBool { .. } => OpCode::LoadBool,
            Self::Imm { .. } => OpCode::Imm,
            Self::Add { .. } => OpCode::Add,
            Self::Jump { .. } => OpCode::Jump,
            Self::Compare { .. } => OpCode::Compare,
            Self::Move { .. } => OpCode::Move,
            Self::Push(_) => OpCode::Push,
            Self::Pop(_) => OpCode::Pop,
            Self::SysCall(_) => OpCode::SysCall,
        });

        match data {
            Self::Halt => (),
            Self::Nop => (),
            Self::LoadBool { dst, op } => {
                writer = writer.write(dst).write(op);
            }
            Self::Imm { dst, value } => {
                writer = writer.write(dst).write(value);
            }
            Self::Add { dst, lhs, rhs } => {
                writer = writer.write(dst).write(lhs).write(rhs);
            }
            Self::Jump { cond, dst } => {
                // We have some unused bits here
                // We need to write them, in order to skip to the correct bit for the offset
                writer = writer.write(cond).write(dst);
            }
            Self::Compare { lhs, rhs } => {
                writer = writer.write(lhs).write(rhs);
            }
            Self::Move { dst, src } => {
                writer = writer.write(dst).write(src);
            }
            Self::Push(reg) => {
                writer = writer.write(reg);
            }
            Self::Pop(reg) => {
                writer = writer.write(reg);
            }
            Self::SysCall(value) => {
                writer = writer.write(value);
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
            Self::LoadBool { dst, op } => write!(f, "LoadBool {}, {:?}", dst, op),
            Self::Imm { dst, value } => write!(f, "Imm {}, {}", dst, value.0),
            Self::Add { dst, lhs, rhs } => write!(f, "Add {}, {}, {}", dst, lhs, rhs),
            Self::Jump { cond, dst } => write!(f, "Jump {:?} {}", cond, dst),
            Self::Compare { lhs, rhs } => write!(f, "Compare {}, {}", lhs, rhs),
            Self::Move { dst, src } => write!(f, "Move {} => {}", src, dst),
            Self::Push(reg) => write!(f, "Push {}", reg),
            Self::Pop(reg) => write!(f, "Pop {}", reg),
            Self::SysCall(value) => write!(f, "SysCall {:?}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn jump_to_bytes() {
        use super::*;
        use args::jump_cond::JumpCondition;

        // Should look like this
        // 001001_000000_11111111111111010100

        // 11111111111111111111111111010100
        let offset = -44i32;

        println!("Offset: {:b}", offset);

        let instruction = Instruction::Jump {
            cond: JumpCondition::Always,
            dst: (offset as u32).into(),
        };

        let bytes = Instruction::match_to_bytes(instruction);

        assert_eq!(bytes, 0b001001_000000_11111111111111010100);
    }
}
