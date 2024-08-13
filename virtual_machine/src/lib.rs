//! This module contains the virtual machine implementation.
//! The virtual machine is responsible for executing the bytecode
//! generated by the compiler.
//!
//! The virtual machine is also register-based, meaning that it uses
//! registers to store intermediate values during the execution of the
//! program.
//!
//!
//!
//! 31            26 25       20 19                                0
//! ┌───────────────┬───────────┬───────────────────────────────────┐
//! │     OPCODE    │    REG    │              OTHER ARGS           │
//! └───────────────┴───────────┴───────────────────────────────────┘

use error::VMResult;
use instruction::InstructionReader;
use memory::Memory;
use opcode::OpCode;
use register::{Register, RegisterStore};

pub mod error;
pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod register;
pub struct Machine {
    memory: Box<dyn Memory>,
    registers: RegisterStore,

    halt: bool,
}

impl Machine {
    pub fn new(memory: impl Memory + 'static) -> Machine {
        let mut res = Self {
            memory: Box::new(memory),
            registers: RegisterStore::default(),
            halt: false,
        };
        res.reset_registers();
        res
    }

    pub fn reset_registers(&mut self) {
        self.registers = RegisterStore::default();
        self.registers.set(Register::IP, 3000);
    }

    pub fn run(&mut self) -> VMResult<()> {
        while !self.halt {
            self.step()?;
        }
        Ok(())
    }

    fn step(&mut self) -> VMResult<()> {
        if self.halt {
            return Ok(());
        }

        let instruction_pointer = self.registers.get(Register::IP);
        let instruction = self.memory.read(instruction_pointer)?;

        self.registers.set(Register::IP, instruction_pointer + 1);

        self.run_instruction(&instruction)?;

        Ok(())
    }

    fn run_instruction(&mut self, instruction: &u32) -> VMResult<()> {
        let mut reader = InstructionReader::new(*instruction);
        let op_code = reader.read::<OpCode>()?;
        match op_code {
            OpCode::Halt => {
                self.halt = true;
                Ok(())
            }
            OpCode::Nop => Ok(()),
            OpCode::Load => instruction::load(&mut reader, self),
            OpCode::Imm => instruction::imm(&mut reader, self),
        }
    }

    pub fn registers(&self) -> &RegisterStore {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut RegisterStore {
        &mut self.registers
    }
}

pub fn sign_extend(value: u32, from: u32) -> u32 {
    if (value >> (from - 1)) & 1 != 0 {
        value | (0xffffffff << from)
    } else {
        value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sign_extend() {
        assert_eq!(
            sign_extend(0b00000000000000000000000000011111, 5) as i32,
            -1
        );
    }
}
