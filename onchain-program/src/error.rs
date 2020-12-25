//! Error type


use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, 
    program_error::ProgramError,
    msg,
    program_error::PrintProgramError};
use thiserror::Error;
use num_traits::FromPrimitive;

/// Errors that may be returned by the solong-lottery program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum LotteryError {
    /// Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,

    /// Invalid pool length
    #[error("Invalid pool length")]
    InvalidPoolLength,

    /// Insufficent funds
    #[error("Insufficent funds")]
    InsufficentFunds,

    /// Invalid permission
    #[error("Invalid permission")]
    InvalidPermission,

    /// Invalid account length
    #[error("Invalid account length")]
    InvalidAccountLength,

    /// Already signin
    #[error("Already signin")]
    AlreadySignin, 

    /// Invalied fee account
    #[error("Invalied fee account")]
    InvaliedFee, 

    /// Low balance for 1 SOL
    #[error("Low balance for 1 SOL")]
    LowBalance, 

    /// Invalid account for reward 
    #[error("Invalid account for reward")]
    InvalidAccountForReward, 


    /// too many players
    #[error("Too many players")]
    TooManyPlayers, 

    /// TestError 
    #[error("TestError")]
    TestError,
}
impl From<LotteryError> for ProgramError {
    fn from(e: LotteryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for LotteryError {
    fn type_of() -> &'static str {
        "LotteryError"
    }
}


impl PrintProgramError for LotteryError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            LotteryError::InvalidInstruction => msg!("Invalid instruction"),
            LotteryError::InvalidPoolLength => msg!("Invalid pool length"),
            LotteryError::InsufficentFunds => msg!("Insufficent funds"),
            LotteryError::InvalidPermission=> msg!("Invalid permission"),
            LotteryError::InvalidAccountLength=> msg!("Invalid account length"),
            LotteryError::AlreadySignin=> msg!("Already signin"),
            LotteryError::InvaliedFee=> msg!("Invalied fee account"),
            LotteryError::LowBalance=> msg!("Low balance for 1 SOL"),
            LotteryError::InvalidAccountForReward=> msg!("Invalid account for reward"),
            LotteryError::TooManyPlayers=> msg!("Too many players"),
            LotteryError::TestError => msg!("TestError"),
        }
    }
}