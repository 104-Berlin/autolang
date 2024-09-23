use std::ops::Deref;

use crate::{error::VMResult, sign_extend};

use super::InstructionArg;

#[derive(Debug)]
pub struct ArgN<const N: u8>(pub u32);

impl<const N: u8> ArgN<N> {
    pub fn sign_extend(&self) -> u32 {
        sign_extend(self.0, Self::BIT_SIZE)
    }
}

impl<const N: u8> InstructionArg for ArgN<N> {
    const BIT_SIZE: u32 = N as u32;

    fn match_to_bytes(data: Self) -> u32 {
        data.0 & Self::MASK
    }

    fn match_from_bytes(data: u32) -> VMResult<Self> {
        Ok(Self(data & Self::MASK))
    }
}

impl<const N: u8> From<u32> for ArgN<N> {
    fn from(data: u32) -> Self {
        Self(data)
    }
}

impl<const N: u8> Deref for ArgN<N> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Arg2 = ArgN<2>;
pub type Arg6 = ArgN<6>;
pub type Arg8 = ArgN<8>;
pub type Arg12 = ArgN<12>;
pub type Arg16 = ArgN<16>;
pub type Arg18 = ArgN<18>;
pub type Arg20 = ArgN<20>;
pub type Arg26 = ArgN<26>;
