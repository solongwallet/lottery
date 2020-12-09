//! Instruction types

use crate::{
    error::LotteryError,
};
use solana_program::{
    program_error::ProgramError,
};
use std::mem::size_of;

/// Instructions supported by the solong-lottery program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum LotteryInstruction {
    /// Initialize Instruction
    Initialize, 

    /// SignIn Instruction
    SignIn,

    /// Buy Instrcution
    Buy,

    /// Roll Instruction
    Roll,
}


impl LotteryInstruction {
    /// Unpacks a byte buffer into a [RegistryInstruction](enum.RegistryInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use LotteryError::InvalidInstruction;

        let (&tag, _rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag { 
            1 => Self::Initialize,
            2 => Self::SignIn,
            3 => Self::Buy,
            4 => Self::Roll,
            _ => return Err(LotteryError::InvalidInstruction.into()),
        })
    }


    /// Packs a [RegistryInstruction](enum.RegistryInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf : Vec<u8>;
        let self_len= size_of::<Self>();
        match &*self {
            Self::Initialize => {
                buf = Vec::with_capacity(self_len);
                buf.push(1); 
            }

            Self::SignIn => {
                buf = Vec::with_capacity(self_len);
                buf.push(2); 
            }

            Self::Buy=> {
                buf = Vec::with_capacity(self_len);
                buf.push(3); 
            }

            Self::Roll=> {
                buf = Vec::with_capacity(self_len);
                buf.push(4); 
            }
        };
        buf
    }    
}
