use crate::error::VMResult;
use crate::instruction::args::logical_operator::LogicalOperator;
use crate::instruction::args::register_or_register_pointer::RegisterOrRegisterPointer;
use crate::instruction::{
    args::{
        arg20::Arg20, jump_cond::JumpCondition, register_or_literal::RegisterOrLiteral,
        InstructionArg,
    },
    Instruction,
};
use crate::memory::Memory;
use crate::register::{ConditionFlag, Register, RegisterStore};
use crate::sign_extend;

pub struct Machine {
    memory: Box<dyn Memory>,
    registers: RegisterStore,

    halt: bool,
    cycle_changed_pc: bool,
}

impl Default for Machine {
    fn default() -> Machine {
        let mut res = Self {
            memory: Box::new(vec![0u32; 1024 * 1024 * 4]),
            registers: RegisterStore::default(),
            halt: false,
            cycle_changed_pc: false,
        };
        res.reset_registers();
        res
    }
}

impl Machine {
    pub const STACK_START: u32 = 0x0300; // Musst be devisable by 4
    pub const PROGRAM_START: u32 = 0x0BBC; // Musst be devisable by 4

    pub fn load_program(mut self, program: &[u32]) -> VMResult<Self> {
        for (i, instr) in program.iter().enumerate() {
            self.memory
                .write(Self::PROGRAM_START + (i * 4) as u32, *instr)?;
        }

        Ok(self)
    }

    pub fn reset_registers(&mut self) {
        self.registers = RegisterStore::default();
        self.registers.set(Register::PC, Self::PROGRAM_START);
        self.registers.set(Register::SP, Self::STACK_START);
    }

    pub fn run(mut self, step_mode: bool) -> VMResult<Self> {
        self.memory
            .dump(Self::PROGRAM_START as usize..Self::PROGRAM_START as usize + 32);
        while !self.halt {
            self.step()?;
            if step_mode {
                println!("{}", self.registers);
                println!("Press enter to continue...");
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
        Ok(self)
    }

    fn step(&mut self) -> VMResult<()> {
        if self.halt {
            return Ok(());
        }

        let instruction_pointer = self.registers.get(Register::PC);

        let instruction = self.memory.read(instruction_pointer)?;

        self.cycle_changed_pc = false;

        self.run_instruction(&instruction)?;

        if !self.cycle_changed_pc {
            self.registers.set(Register::PC, instruction_pointer + 4);
        }

        Ok(())
    }

    fn run_instruction(&mut self, instruction: &u32) -> VMResult<()> {
        let instr = Instruction::match_from_bytes(*instruction)?;
        println!("Running instruction {}", instr);

        match instr {
            Instruction::Halt => self.halt = true,
            Instruction::Nop => (),
            Instruction::LoadBool { dst, op } => self.load_bool(dst, op)?,
            Instruction::Imm { dst, value } => self.imm(dst, value),
            Instruction::Add { dst, lhs, rhs } => self.add(dst, lhs, rhs),
            Instruction::Jump { cond, offset } => {
                let cond_flags = self.registers.get(Register::Cond) as u8;
                let cond_flags: ConditionFlag = cond_flags
                    .try_into()
                    .expect("There is a wrong value in the condition register!");

                let can_jump = matches!(
                    (cond, cond_flags),
                    (JumpCondition::Always, _)
                        | (JumpCondition::Zero, ConditionFlag::Zero)
                        | (JumpCondition::Positive, ConditionFlag::Positive)
                        | (JumpCondition::Negative, ConditionFlag::Negative)
                        | (JumpCondition::NotZero, ConditionFlag::Positive)
                        | (JumpCondition::NotZero, ConditionFlag::Negative)
                );

                if can_jump {
                    let pc = self.registers.get(Register::PC);
                    self.registers.set(
                        Register::PC,
                        (pc as i32 + sign_extend(*offset, 20) as i32) as u32,
                    );
                    self.cycle_changed_pc = true;
                }
            }
            Instruction::Compare { lhs, rhs } => {
                let lhs = lhs.get_val(self);
                let rhs = rhs.get_val(self);

                // Store the result in RS1 to update the condition flags
                self.registers.set(Register::RSC, lhs.wrapping_sub(rhs));
                self.registers.update_condition(Register::RSC);
            }
            Instruction::Move { dst, src } => {
                self.mov(dst, src)?;
            }
            Instruction::Push(reg) => {
                let sp = self.registers.get(Register::SP);
                let reg_val = reg.get_val(self);
                self.memory.write(sp, reg_val)?;
                self.registers.set(Register::SP, sp + 4);
            }
            Instruction::Pop(reg) => {
                let sp = self.registers.get(Register::SP);
                let reg_val = self.memory.read(sp - 4)?;
                self.registers.set(reg, reg_val);
                self.registers.set(Register::SP, sp - 4);
            }
        }

        Ok(())
    }

    pub fn registers(&self) -> &RegisterStore {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut RegisterStore {
        &mut self.registers
    }

    pub fn memory(&self) -> &dyn Memory {
        &*self.memory
    }

    pub fn memory_mut(&mut self) -> &mut dyn Memory {
        &mut *self.memory
    }

    fn mov(
        &mut self,
        dst: RegisterOrRegisterPointer,
        src: RegisterOrRegisterPointer,
    ) -> VMResult<()> {
        dst.write(self, src.read(self)?)?;
        Ok(())
    }

    fn load_bool(&mut self, dst: Register, operator: LogicalOperator) -> VMResult<()> {
        let cond_flags = self.registers.get(Register::Cond) as u8;
        let cond_flags: ConditionFlag = cond_flags
            .try_into()
            .expect("There is a wrong value in the condition register!");
        match (cond_flags, operator) {
            (ConditionFlag::Zero, LogicalOperator::EQ)
            | (ConditionFlag::Positive, LogicalOperator::NE)
            | (ConditionFlag::Negative, LogicalOperator::NE)
            | (ConditionFlag::Negative, LogicalOperator::LT)
            | (ConditionFlag::Positive, LogicalOperator::GT)
            | (ConditionFlag::Negative, LogicalOperator::LE)
            | (ConditionFlag::Positive, LogicalOperator::GE) => {
                self.registers.set(dst, 1);
            }
            _ => {
                self.registers.set(dst, 0);
            }
        }
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

    pub fn dump_stack(&self) {
        let end = self.registers.get(Register::SP);
        let mut sp = Self::STACK_START;
        while sp < end {
            let val = self.memory.read(sp).unwrap();
            println!("0x{:04x}: {}", sp, val);
            sp += 4;
        }
    }
}
