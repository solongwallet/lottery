//! Program state processor

use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::LotteryState,
    log_info,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
};


/// Program state handler.
pub struct Processor {}
impl Processor {

    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        log_info("process program:");
        let instruction = LotteryInstruction::unpack(input)?;

        match instruction {
            LotteryInstruction::Initialize{
                price
            }=>{
                log_info("LotteryInstruction::Initialize");
                Self::process_initialize(program_id, accounts, price)
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
        price:u64,
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let config_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;
        let fee_info= next_account_info(account_info_iter)?;

        //TODO: check permission first
    

        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        lottery.price = price;
        lottery.fee =  *fee_info.key;
        lottery.pool.clear();
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;
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
        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        let key = lottery.pool.entry(*account_info.key).or_insert(0);
        *key += 1;
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;
        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_buy(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let system_program_info= next_account_info(account_info_iter)?;
        let config_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;
        let fee_info= next_account_info(account_info_iter)?;
        let account_info= next_account_info(account_info_iter)?;

        let price_lamports = 1000_000_000;
        // need not check balance Cau'z it will fail
        invoke(
            &system_instruction::transfer(
                account_info.key,
                fee_info.key,
                price_lamports,
            ),
            &[
                account_info.clone(),
                fee_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        let mut  lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        let key = lottery.pool.entry(*account_info.key).or_insert(0);
        *key += 1;
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;
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
