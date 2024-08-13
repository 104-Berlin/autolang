use crate::{
    error::{VMError, VMResult},
    opcode::OpCode,
    register::Register,
    sign_extend, Machine,
};

pub trait InstructionPart {
    const BIT_SIZE: u32;

    /// Match the instruction from the bytes
    /// The bit_offset is used to skip the bits that are not part of the instruction
    /// We are cutting all the bits that are not part of the instruction and put the instruction in the first bits
    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized;

    /// Match the instruction to the bytes
    /// This is used to convert the instruction back to the bytes
    /// The data should be on the most right side with BIT_SIZE bits
    fn match_to_bytes(data: Self) -> u32;

    fn from_instruction(instruction: u32, bit_offset: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        if bit_offset + Self::BIT_SIZE > 32 {
            return Err(VMError::FailedParsingInstruction(instruction));
        }
        let offset = 32 - (Self::BIT_SIZE + bit_offset);
        let code = (instruction >> offset) & ((1 << Self::BIT_SIZE) - 1);

        Self::match_from_bytes(code)
    }

    fn into_instruction(instruction: &mut u32, bit_offset: u32, data: Self)
    where
        Self: Sized,
    {
        *instruction |= Self::match_to_bytes(data) << (32 - (Self::BIT_SIZE + bit_offset));
    }
}

pub struct InstructionReader {
    instruction: u32,
    bit_offset: u32,
}

impl InstructionReader {
    pub fn new(instruction: u32) -> Self {
        Self {
            instruction,
            bit_offset: 0,
        }
    }

    pub fn read<T: InstructionPart>(&mut self) -> VMResult<T> {
        let result = T::from_instruction(self.instruction, self.bit_offset)?;
        self.bit_offset += T::BIT_SIZE;
        Ok(result)
    }
}

pub struct InstructionWriter {
    instruction: u32,
    bit_offset: u32,
}

impl InstructionWriter {
    pub fn new(op_code: OpCode) -> Self {
        Self {
            instruction: 0,
            bit_offset: 0,
        }
        .write::<OpCode>(op_code)
    }

    pub fn write<T: InstructionPart>(mut self, data: T) -> Self {
        T::into_instruction(&mut self.instruction, self.bit_offset, data);
        self.bit_offset += T::BIT_SIZE;
        self
    }

    pub fn finish(self) -> u32 {
        self.instruction
    }
}

#[derive(Debug)]
pub struct Arg20(pub u32);

/// ```text
///    9   8          6 5          0
/// ┌─────┬────────────┬────────────┐
/// │  0  |   UNUSED   |    REGA    │
/// └─────┴────────────┴────────────┘
/// ```
///
/// ```text
///    9         8      7          0
/// ┌─────┬────────────┬────────────┐
/// │  1  |   UNUSED   |  LITERAL8  │
/// └─────┴────────────┴────────────┘
/// ```
#[derive(Debug)]
pub enum RegisterOrLiteral {
    Register(Register),
    Literal(u8),
}

pub struct Unused<const T: u32>;

impl<const T: u32> InstructionPart for Unused<T> {
    const BIT_SIZE: u32 = T;

    fn match_from_bytes(_: u32) -> VMResult<Self> {
        Ok(Self)
    }

    fn match_to_bytes(_: Self) -> u32 {
        0
    }
}

impl RegisterOrLiteral {
    pub fn get_val(&self, machine: &Machine) -> u32 {
        match self {
            Self::Register(register) => machine.registers().get(*register),
            Self::Literal(literal) => sign_extend(*literal as u32, 8),
        }
    }
}

impl From<Register> for RegisterOrLiteral {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl InstructionPart for Arg20 {
    const BIT_SIZE: u32 = 20;

    fn match_to_bytes(data: Self) -> u32 {
        data.0 & 0xF_FFFF
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(Self(data & 0xF_FFFF))
    }
}

impl InstructionPart for RegisterOrLiteral {
    const BIT_SIZE: u32 = 10;

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let is_literal = (data >> 9) & 0x1 != 0;
        if is_literal {
            Ok(Self::Literal((data & 0xFF) as u8))
        } else {
            Ok(Self::Register(Register::match_from_bytes(data)?))
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        match data {
            Self::Register(register) => register as u32,
            Self::Literal(literal) => 0x200 | (literal as u32),
        }
    }
}

impl InstructionPart for bool {
    const BIT_SIZE: u32 = 1;

    fn match_to_bytes(data: Self) -> u32 {
        data as u32
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(data & 0x1 != 0)
    }
}

impl InstructionPart for u8 {
    const BIT_SIZE: u32 = 8;

    fn match_to_bytes(data: Self) -> u32 {
        data as u32
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(data as u8)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Halt,
    Nop,
    Load(Register, Arg20),
    Imm(Register, Arg20),
    Add(Register, RegisterOrLiteral, RegisterOrLiteral),
}

impl InstructionPart for Instruction {
    const BIT_SIZE: u32 = 26;

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
            OpCode::Add => {
                println!("Reading Add {:032b}", data);
                Ok(Self::Add(reader.read()?, reader.read()?, reader.read()?))
            }
        }
    }

    fn match_to_bytes(data: Self) -> u32 {
        let mut writer = InstructionWriter::new(match data {
            Self::Halt => OpCode::Halt,
            Self::Nop => OpCode::Nop,
            Self::Load(_, _) => OpCode::Load,
            Self::Imm(_, _) => OpCode::Imm,
            Self::Add(_, _, _) => OpCode::Add,
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
        }

        writer.finish()
    }
}
