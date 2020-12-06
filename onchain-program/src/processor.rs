//! Program state processor

use crate::{
    error::LotteryError,
    instruction::{LotteryInstruction},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    //decode_error::DecodeError,
    program_error::ProgramError,
    entrypoint::ProgramResult,
    info,
    program_option::COption,
    //program_pack::{IsInitialized, Pack},
    program_pack::{Pack},
    pubkey::Pubkey,
    //sysvar::{rent::Rent, Sysvar},
    //sysvar::{rent::Rent},
};

/// Program state handler.
pub struct Processor {}
impl Processor {

    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        info!("solong-lottery:process program:");
        let instruction = LotteryInstruction::unpack(input)?;

        match instruction {
            LotteryInstruction::SignIn => {
                info!("solong-lottery: Instruction: SignIn");
                return Ok(());
            }
        }
    }
 
}
