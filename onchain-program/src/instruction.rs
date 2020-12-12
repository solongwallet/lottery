//! Instruction types

use crate::{
    error::LotteryError,
};
use solana_program::{
    program_error::ProgramError,
};
use std::mem::size_of;
use std::convert::TryInto;

/// Instructions supported by the solong-lottery program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum LotteryInstruction {
    /// Initialize Instruction
    Initialize {
        /// fund from solong
        fund : u64,
        /// price of lottery , unit lamports
        price : u64,
    }, 

    /// SignIn Instruction
    SignIn,

    /// Buy Instrcution
    Buy,

    /// Roll Instruction
    Roll,

    /// Reward Instruction
    Reward,
}


impl LotteryInstruction {
    /// Unpacks a byte buffer into a [RegistryInstruction](enum.RegistryInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use LotteryError::InvalidInstruction;

        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag { 
            1 => {
                let (fund, rest) = Self::unpack_u64(rest)?;
                let (price, _) = Self::unpack_u64(rest)?;
                Self::Initialize{
                    fund,
                    price
                }
            } 
            2 => Self::SignIn,
            3 => Self::Buy,
            4 => Self::Roll,
            5 => Self::Reward,
            _ => return Err(LotteryError::InvalidInstruction.into()),
        })
    }


    /// Packs a [RegistryInstruction](enum.RegistryInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf : Vec<u8>;
        let self_len= size_of::<Self>();
        match &*self {
            Self::Initialize {
                fund,
                price,
            } => {
                buf = Vec::with_capacity(self_len);
                buf.push(1); 
                buf.extend_from_slice(&fund.to_le_bytes());
                buf.extend_from_slice(&price.to_le_bytes());
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

            Self::Reward => {
                buf = Vec::with_capacity(self_len);
                buf.push(5); 
            }
        };
        buf
    }    



    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(LotteryError::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(LotteryError::InvalidInstruction.into())
        }
    }
}
