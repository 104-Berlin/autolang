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
use instruction::{Arg20, Instruction, InstructionPart, RegisterOrLiteral};
use memory::Memory;
use register::{Register, RegisterStore};

pub mod error;
pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod program_builder;
pub mod register;
pub struct Machine {
    memory: Box<dyn Memory>,
    registers: RegisterStore,

    halt: bool,
}

impl Machine {
    pub fn new(memory: Box<dyn Memory>) -> Machine {
        let mut res = Self {
            memory,
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

    pub fn run(mut self) -> VMResult<Self> {
        while !self.halt {
            self.step()?;
        }
        Ok(self)
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
        let instr = Instruction::match_from_bytes(*instruction)?;
        println!("Running instruction {:?}", instr);

        match instr {
            Instruction::Halt => self.halt = true,
            Instruction::Nop => (),
            Instruction::Load(dst, offset) => self.load(dst, offset)?,
            Instruction::Imm(dst, val) => self.imm(dst, val),
            Instruction::Add(dst, a1, a2) => self.add(dst, a1, a2),
        }

        Ok(())
    }

    pub fn registers(&self) -> &RegisterStore {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut RegisterStore {
        &mut self.registers
    }

    fn load(&mut self, dst: Register, offset: Arg20) -> VMResult<()> {
        let ip = self.registers.get(Register::IP);
        let addr = (ip as u64 + sign_extend(offset.0, 20) as u64) as u32;
        let addr = self.memory.read(addr)?;
        self.registers.set(dst, addr);
        self.registers.update_condition(dst);
        Ok(())
    }

    fn imm(&mut self, dst: Register, value: Arg20) {
        self.registers.set(dst, sign_extend(value.0, 20));
        self.registers.update_condition(dst);
    }

    fn add(&mut self, dst: Register, a: RegisterOrLiteral, b: RegisterOrLiteral) {
        self.registers
            .set(dst, a.get_val(self).wrapping_add(b.get_val(self)));
        self.registers.update_condition(dst);
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
