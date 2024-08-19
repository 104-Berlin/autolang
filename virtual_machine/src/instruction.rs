use std::fmt::Display;

use args::{arg20::Arg20, register_or_literal::RegisterOrLiteral, unused::Unused, InstructionArg};
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
    Load(Register, Arg20),
    Imm(Register, Arg20),
    Add(Register, RegisterOrLiteral, RegisterOrLiteral),
    Compare(Register, RegisterOrLiteral),
    Jump(Arg20),
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
            OpCode::Load => Ok(Self::Load(reader.read()?, reader.read()?)),
            OpCode::Imm => Ok(Self::Imm(reader.read()?, reader.read()?)),
            OpCode::Add => Ok(Self::Add(reader.read()?, reader.read()?, reader.read()?)),
            OpCode::Jump => {
                reader.read::<Unused<{ Register::BIT_SIZE }>>()?;
                Ok(Self::Jump(reader.read()?))
            }
            OpCode::Compare => Ok(Self::Compare(reader.read()?, reader.read()?)),
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        let mut writer = InstructionWriter::new(match data {
            Self::Halt => OpCode::Halt,
            Self::Nop => OpCode::Nop,
            Self::Load(_, _) => OpCode::Load,
            Self::Imm(_, _) => OpCode::Imm,
            Self::Add(_, _, _) => OpCode::Add,
            Self::Jump(_) => OpCode::Jump,
            Self::Compare(_, _) => OpCode::Compare,
        });

        match data {
            Self::Halt => (),
            Self::Nop => (),
            Self::Load(dst, offset) => {
                writer = writer.write(dst).write(offset);
            }
            Self::Imm(dst, val) => {
                writer = writer.write(dst).write(val);
            }
            Self::Add(dst, a1, a2) => {
                writer = writer.write(dst).write(a1).write(a2);
            }
            Self::Jump(offset) => {
                // We have some unused bits here
                // We need to write them, in order to skip to the correct bit for the offset
                writer = writer.write(Unused::<{ Register::BIT_SIZE }>).write(offset);
            }
            Self::Compare(lhs, rhs) => {
                writer = writer.write(lhs).write(rhs);
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
            Self::Load(dst, offset) => write!(f, "Load {}, {}", dst, offset.0),
            Self::Imm(dst, val) => write!(f, "Imm {}, {}", dst, val.0),
            Self::Add(dst, a1, a2) => write!(f, "Add {}, {}, {}", dst, a1, a2),
            Self::Jump(offset) => write!(f, "Jump {}", offset.0 as i32),
            Self::Compare(lhs, rhs) => write!(f, "Compare {}, {}", lhs, rhs),
        }
    }
}
