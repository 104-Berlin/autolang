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

pub struct Arg20(pub u32);

impl InstructionPart for Arg20 {
    const BIT_SIZE: u32 = 20;

    fn match_to_bytes(data: Self) -> u32 {
        data.0 & 0xF_FFFF
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(Self(data & 0xF_FFFF))
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
/// ```text
/// 31            26 25       20 19                                0
/// ┌───────────────┬───────────┬───────────────────────────────────┐
/// │   0b00000002  │    REG    │            PCOFFSET20             │
/// └───────────────┴───────────┴───────────────────────────────────┘
/// ```
pub fn load(reader: &mut InstructionReader, vm: &mut Machine) -> VMResult<()> {
    let register = reader.read::<Register>()?;
    let value = sign_extend(reader.read::<Arg20>()?.0, 20);

    let ip = vm.registers().get(Register::IP);
    let addr = vm.memory.read((ip as u64 + value as u64) as u32)?;

    vm.registers_mut().set(register, addr);
    vm.registers_mut().update_condition(register);

    Ok(())
}

/// ```text
/// 31            26 25       20 19                                0
/// ┌───────────────┬───────────┬───────────────────────────────────┐
/// │   0b00000003  │    REG    │             LITERAL20             │
/// └───────────────┴───────────┴───────────────────────────────────┘
/// ```
pub fn imm(reader: &mut InstructionReader, vm: &mut Machine) -> VMResult<()> {
    let register = reader.read::<Register>()?;
    let value = sign_extend(reader.read::<Arg20>()?.0, 20);

    vm.registers_mut().set(register, value);
    vm.registers_mut().update_condition(register);

    Ok(())
}

/// ```text
/// 31            26 25       20 19             10 9              0
/// ┌───────────────┬───────────┬─────────────────┬────────────────┐
/// │   0b00000004  │   DREG    │     VALUE1      |     VALUE2     │
/// └───────────────┴───────────┴─────────────────┴────────────────┘
/// ```
///
///
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
/// │  0  |   UNUSED   |  LITERAL8  │
/// └─────┴────────────┴────────────┘
/// ```
pub fn add(reader: &mut InstructionReader, vm: &mut Machine) -> VMResult<()> {
    let reg: Register = reader.read()?;

    let read_value = |reader: &mut InstructionReader| {
        let is_literal: bool = reader.read()?;

        Ok::<_, VMError>(if is_literal {
            sign_extend(reader.read::<u8>()? as u32, 8)
        } else {
            vm.registers().get(reader.read::<Register>()?)
        })
    };

    let value1 = read_value(reader)?;
    let value2 = read_value(reader)?;

    vm.registers_mut().set(reg, value1.wrapping_add(value2));

    Ok(())
}
