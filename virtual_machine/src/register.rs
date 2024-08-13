use std::fmt::Display;

use prettytable::row;

use crate::{
    error::{VMError, VMResult},
    instruction::InstructionPart,
};

/// # 6 Bit
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    // General Purpose Registers
    RA1,
    RA2,
    RA3,
    RA4,
    RA5,
    RA6,
    // System registers
    RS1,
    RS2,
    // Instruction Pointer
    IP,
    Cond,
}

impl InstructionPart for Register {
    const BIT_SIZE: u32 = 6; // We need to change representation if we grow past 8 bits (Should not happen)

    fn match_to_bytes(data: Self) -> u32 {
        data as u32
    }

    fn match_from_bytes(data: u32) -> VMResult<Self>
    where
        Self: Sized,
    {
        let data = data as u8;
        match data {
            0 => Ok(Register::RA1),
            1 => Ok(Register::RA2),
            2 => Ok(Register::RA3),
            3 => Ok(Register::RA4),
            4 => Ok(Register::RA5),
            5 => Ok(Register::RA6),
            6 => Ok(Register::RS1),
            7 => Ok(Register::RS2),
            8 => Ok(Register::IP),
            _ => Err(VMError::InvalidRegister(data)),
        }
    }
}

/// RegisterStore is a struct that holds the values of all the registers.
/// All registers are 64-bit wide.
#[derive(Default)]
pub struct RegisterStore {
    // Generatl purpose register
    ra1: u32,
    ra2: u32,
    ra3: u32,
    ra4: u32,
    ra5: u32,
    ra6: u32,
    // System register
    rs1: u32,
    rs2: u32,

    // Instruction pointer
    ip: u32,

    // Condition register
    // State of last operation
    // ZERO, NEGATIVE, POSITIVE
    cond: u32,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ConditionFlag {
    Zero,
    Negative,
    Positive,
}

impl RegisterStore {
    pub fn get(&self, register: Register) -> u32 {
        match register {
            Register::RA1 => self.ra1,
            Register::RA2 => self.ra2,
            Register::RA3 => self.ra3,
            Register::RA4 => self.ra4,
            Register::RA5 => self.ra5,
            Register::RA6 => self.ra6,
            Register::RS1 => self.rs1,
            Register::RS2 => self.rs2,
            Register::IP => self.ip,
            Register::Cond => self.cond,
        }
    }

    pub fn set(&mut self, register: Register, value: u32) {
        match register {
            Register::RA1 => self.ra1 = value,
            Register::RA2 => self.ra2 = value,
            Register::RA3 => self.ra3 = value,
            Register::RA4 => self.ra4 = value,
            Register::RA5 => self.ra5 = value,
            Register::RA6 => self.ra6 = value,
            Register::RS1 => self.rs1 = value,
            Register::RS2 => self.rs2 = value,
            Register::IP => self.ip = value,
            Register::Cond => self.cond = value,
        };
    }

    pub fn update_condition(&mut self, register: Register) {
        if self.get(register) == 0 {
            self.cond = ConditionFlag::Zero as u32;
        } else if self.get(register) & 0x8000_0000 != 0 {
            self.cond = ConditionFlag::Negative as u32;
        } else {
            self.cond = ConditionFlag::Positive as u32;
        }
    }
}

impl Display for RegisterStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable::{Cell, Row, Table};

        let mut table = Table::new();
        table.add_row(row!["Register", "Hex", "Dec", "Bin"]);
        table.add_row(Row::new(vec![
            Cell::new("RA1"),
            Cell::new(&format!("{:#X}", self.ra1)),
            Cell::new(&format!("{}", self.ra1)),
            Cell::new(&format!("{:b}", self.ra1)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RA2"),
            Cell::new(&format!("{:#X}", self.ra2)),
            Cell::new(&format!("{}", self.ra2)),
            Cell::new(&format!("{:b}", self.ra2)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RA3"),
            Cell::new(&format!("{:#X}", self.ra3)),
            Cell::new(&format!("{}", self.ra3)),
            Cell::new(&format!("{:b}", self.ra3)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RA4"),
            Cell::new(&format!("{:#X}", self.ra4)),
            Cell::new(&format!("{}", self.ra4)),
            Cell::new(&format!("{:b}", self.ra4)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RA5"),
            Cell::new(&format!("{:#X}", self.ra5)),
            Cell::new(&format!("{}", self.ra5)),
            Cell::new(&format!("{:b}", self.ra5)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RA6"),
            Cell::new(&format!("{:#X}", self.ra6)),
            Cell::new(&format!("{}", self.ra6)),
            Cell::new(&format!("{:b}", self.ra6)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RS1"),
            Cell::new(&format!("{:#X}", self.rs1)),
            Cell::new(&format!("{}", self.rs1)),
            Cell::new(&format!("{:b}", self.rs1)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("RS2"),
            Cell::new(&format!("{:#X}", self.rs2)),
            Cell::new(&format!("{}", self.rs2)),
            Cell::new(&format!("{:b}", self.rs2)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("IP"),
            Cell::new(&format!("{:#X}", self.ip)),
            Cell::new(&format!("{}", self.ip)),
            Cell::new(&format!("{:b}", self.ip)),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Cond"),
            Cell::new(&format!("{:#X}", self.cond)),
            Cell::new(&format!("{}", self.cond)),
            Cell::new(&format!("{:b}", self.cond)),
        ]));

        write!(f, "{}", table)
    }
}
