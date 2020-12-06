//! Instruction types

use crate::error::LotteryError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    //program_option::COption,
    pubkey::Pubkey,
    info,
    //sysvar,
};
//use std::convert::TryInto;
use std::mem::size_of;
use std::str::from_utf8;

/// Instructions supported by the solong-lottery program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum LotteryInstruction {
    /// SignIn Instruction
    SignIn,
}


impl LotteryInstruction {
    /// Unpacks a byte buffer into a [RegistryInstruction](enum.RegistryInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use LotteryError::InvalidInstruction;

        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag { 
            2 => Self::SignIn,
            _ => return Err(LotteryError::InvalidInstruction.into()),
        })
    }


    /// Packs a [RegistryInstruction](enum.RegistryInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf : Vec<u8>;
        let self_len= size_of::<Self>();
        match self {
            Self::SignIn => {
                buf = Vec::with_capacity(self_len);
                buf.push(2); //tag
            }
        };
        buf
    }    
}
