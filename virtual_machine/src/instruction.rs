use crate::{
    error::{VMError, VMResult},
    opcode::OpCode,
    register::Register,
    sign_extend, Machine,
};

pub trait InstructionPart {
    type Output;

    const BIT_SIZE: u32;

    /// Match the instruction from the bytes
    /// The bit_offset is used to skip the bits that are not part of the instruction
    /// We are cutting all the bits that are not part of the instruction and put the instruction in the first bits
    fn match_from_bytes(data: u32) -> VMResult<Self::Output>;

    /// Match the instruction to the bytes
    /// This is used to convert the instruction back to the bytes
    /// The data should be on the most right side with BIT_SIZE bits
    fn match_to_bytes(data: Self::Output) -> u32;

    fn from_instruction(instruction: u32, bit_offset: u32) -> VMResult<Self::Output> {
        if bit_offset + Self::BIT_SIZE > 32 {
            return Err(VMError::FailedParsingInstruction(instruction));
        }
        let offset = 32 - (Self::BIT_SIZE + bit_offset);
        let code = (instruction >> offset) & ((1 << Self::BIT_SIZE) - 1);

        Self::match_from_bytes(code)
    }

    fn into_instruction(instruction: &mut u32, bit_offset: u32, data: Self::Output) {
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

    pub fn read<T: InstructionPart>(&mut self) -> VMResult<T::Output> {
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

    pub fn write<T: InstructionPart>(mut self, data: T::Output) -> Self {
        T::into_instruction(&mut self.instruction, self.bit_offset, data);
        self.bit_offset += T::BIT_SIZE;
        self
    }

    pub fn finish(self) -> u32 {
        self.instruction
    }
}

pub struct Arg20;

impl InstructionPart for Arg20 {
    type Output = u32;
    const BIT_SIZE: u32 = 20;

    fn match_to_bytes(data: Self::Output) -> u32 {
        data & 0xF_FFFF
    }

    fn match_from_bytes(data: u32) -> VMResult<Self::Output> {
        Ok(data & 0xF_FFFF)
    }
}

/// ```text
/// 31            26 25       20 19                                0
/// ┌───────────────┬───────────┬───────────────────────────────────┐
/// │   0b00000001  │    REG    │               VALUE               │
/// └───────────────┴───────────┴───────────────────────────────────┘
/// ```
pub fn load(reader: &mut InstructionReader, vm: &mut Machine) -> VMResult<()> {
    let register = reader.read::<Register>()?;
    let value = sign_extend(reader.read::<Arg20>()?, 20);

    let ip = vm.registers().get(Register::IP);
    let addr = vm.memory.read((ip as u64 + value as u64) as u32)?;

    vm.registers_mut().set(register, addr);

    Ok(())
}
