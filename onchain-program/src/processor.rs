//! Program state processor

use crate::{
    //error::LotteryError,
    instruction::LotteryInstruction,
    log_info,
};
use solana_program::{
    //account_info::{next_account_info, AccountInfo},
    account_info::AccountInfo,
    //decode_error::DecodeError,
    //program_error::ProgramError,
    entrypoint::ProgramResult,
    //program_option::COption,
    //program_pack::{IsInitialized, Pack},
    //program_pack::{Pack},
    pubkey::Pubkey,
    //sysvar::{rent::Rent, Sysvar},
    //sysvar::{rent::Rent},
};

/// Program state handler.
pub struct Processor {}
impl Processor {

    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        log_info("process program:");
        let instruction = LotteryInstruction::unpack(input)?;

        match instruction {
            LotteryInstruction::Initialize {
                supply
            }=>{
                log_info("LotteryInstruction::Initialize");
                Self::process_initialize(program_id, accounts, supply)
            }

            LotteryInstruction::SignIn => {
                log_info("Instruction: SignIn");
                return Ok(());
            }
        }
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_initialize(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _supply: u64,
    ) -> ProgramResult {
        Ok(())
    }
 
}
