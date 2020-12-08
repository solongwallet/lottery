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
            LotteryError::TestError => msg!("TestError"),
        }
    }
}