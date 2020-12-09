//! Program state processor

use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::LotteryPool,
    log_info,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {

    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        log_info("process program:");
        let instruction = LotteryInstruction::unpack(input)?;

        match instruction {
            LotteryInstruction::Initialize =>{
                log_info("LotteryInstruction::Initialize");
                Self::process_initialize(program_id, accounts)
            }

            LotteryInstruction::SignIn => {
                log_info("Instruction: SignIn");
                Self::process_signin(program_id, accounts)
            }

            LotteryInstruction::Buy => {
                log_info("Instruction: Buy");
                Self::process_buy(program_id, accounts)
            }

            LotteryInstruction::Roll => {
                log_info("Instruction: Roll");
                Self::process_roll(program_id, accounts)
            }
        }
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let config_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_signin(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let config_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;
        let account_info= next_account_info(account_info_iter)?;
        let mut pool = LotteryPool::unpack_unchecked(&pool_info.data.borrow())?;
        //let mut accounts = &pool.accounts;
        let key = pool.accounts.entry(*account_info.key).or_insert(0);
        *key += 1;
        LotteryPool::pack(pool, &mut pool_info.data.borrow_mut())?;
        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_buy(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let config_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;
        let account_info= next_account_info(account_info_iter)?;

        let account_lamports = account_info.lamports();
        if account_lamports < 1000_000_000 {
            return Err(LotteryError::InsufficentFunds.into);
        }
        **account_info.lamports.borrow_mut() = account_lamports - 1000_000_000;

        let mut pool = LotteryPool::unpack_unchecked(&pool_info.data.borrow())?;
        let key = pool.accounts.entry(*account_info.key).or_insert(0);
        *key += 1;
        LotteryPool::pack(pool, &mut pool_info.data.borrow_mut())?;
        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_roll(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let config_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;

        Ok(())
    }
 
}
