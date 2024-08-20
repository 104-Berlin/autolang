use crate::error::VMResult;
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
        self.registers.set(Register::SP, 1000);
    }

    pub fn run(mut self, step_mode: bool) -> VMResult<Self> {
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

        let instruction_pointer = self.registers.get(Register::IP);
        let instruction = self.memory.read(instruction_pointer)?;

        self.registers.set(Register::IP, instruction_pointer + 1);

        self.run_instruction(&instruction)?;

        Ok(())
    }

    fn run_instruction(&mut self, instruction: &u32) -> VMResult<()> {
        let instr = Instruction::match_from_bytes(*instruction)?;
        println!("Running instruction {}", instr);

        match instr {
            Instruction::Halt => self.halt = true,
            Instruction::Nop => (),
            Instruction::Load { dst, addr } => self.load(dst, addr)?,
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
                    let ip = self.registers.get(Register::IP);
                    self.registers.set(
                        Register::IP,
                        (ip as i32 + sign_extend(offset.0, 20) as i32) as u32,
                    );
                }
            }
            Instruction::Compare { lhs, rhs } => {
                let lhs = lhs.get_val(self);
                let rhs = rhs.get_val(self);

                // Store the result in RS1 to update the condition flags
                self.registers.set(Register::RS1, lhs - rhs);
                self.registers.update_condition(Register::RS1);
            }
            Instruction::Push(reg) => {
                let sp = self.registers.get(Register::SP);
                let reg_val = reg.get_val(self);
                self.memory.write(sp, reg_val)?;
                self.registers.set(Register::SP, sp + 1);
            }
            Instruction::Pop(reg) => {
                let sp = self.registers.get(Register::SP);
                let reg_val = self.memory.read(sp - 1)?;
                self.registers.set(reg, reg_val);
                self.registers.set(Register::SP, sp - 1);
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
