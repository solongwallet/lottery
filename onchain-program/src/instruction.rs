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
    SignIn ,

    /// Game Manager command
    GM {
        /// fund from solong
        fund : u64,
        /// price of lottery , unit lamports
        price : u64,
    },

    /// Roll Instruction
    Roll,

    /// Reward Instruction
    Reward,
}


impl LotteryInstruction {
    /// Unpacks a byte buffer into a [LotteryInstruction](enum.LotteryInstruction.html).
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
            3 => {
                let (fund, rest) = Self::unpack_u64(rest)?;
                let (price, _) = Self::unpack_u64(rest)?;
                Self::GM{
                    fund,
                    price
                }
            }
            4 => Self::Roll,
            5 => Self::Reward,
            _ => return Err(LotteryError::InvalidInstruction.into()),
        })
    }


    /// Packs a [LotteryInstruction](enum.LotteryInstruction.html) into a byte buffer.
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

            Self::GM {
                fund,
                price,    
            }=> {
                buf = Vec::with_capacity(self_len);
                buf.push(3); 
                buf.extend_from_slice(&fund.to_le_bytes());
                buf.extend_from_slice(&price.to_le_bytes());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_initialize() {
        let check = LotteryInstruction::Initialize{
            fund:0u64,
            price:0u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 

        let check = LotteryInstruction::Initialize{
            fund:9527u64,
            price:0u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[55, 37, 0, 0, 0, 0, 0, 0]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 

        let check = LotteryInstruction::Initialize{
            fund:10_000_000_000u64,
            price:0u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 

        let check = LotteryInstruction::Initialize{
            fund:10_000_000_000u64,
            price:1_000_000_000u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[0, 202, 154, 59, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 

    }

    #[test]
    fn test_instruction_signin() {
        let check = LotteryInstruction::SignIn;
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[2]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 
    }

    #[test]
    fn test_instruction_gm() {

        let check = LotteryInstruction::Initialize{
            fund:0u64,
            price:0u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = LotteryInstruction::GM{
            fund:0u64,
            price:0u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[3]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0,0,0,0,0,0,0,0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 


        let check = LotteryInstruction::GM{
            fund:10_000_000_000u64,
            price:1_000_000_000u64,
        };
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[3]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[0, 202, 154, 59, 0, 0, 0, 0]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 
    }

    #[test]
    fn test_instruction_roll() {
        let check = LotteryInstruction::Roll;
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[4]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 
    }

    #[test]
    fn test_instruction_reward() {
        let check = LotteryInstruction::Reward;
        let packed = check.pack();
        let mut expect = Vec::new();
        expect.extend_from_slice(&[5]);
        assert_eq!(packed, expect);
        let unpacked = LotteryInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check); 
    }
}
