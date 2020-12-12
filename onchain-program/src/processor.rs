//! Program state processor

use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{LotteryState, AwardState, AwardBill},
    log_info,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
    clock::Clock,
    sysvar::Sysvar,
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
                fund,
                price
            }=>{
                log_info("LotteryInstruction::Initialize");
                Self::process_initialize(program_id, accounts, fund, price)
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

            LotteryInstruction::Reward => {
                log_info("Instruction: Reward");
                Self::process_reward(program_id, accounts)
            }
        }
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        fund:u64,
        price:u64,
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let pool_info= next_account_info(account_info_iter)?;
        let fee_info= next_account_info(account_info_iter)?;
        let billboard_info= next_account_info(account_info_iter)?;
        let admin_info= next_account_info(account_info_iter)?;

        //check permission first
        // TODO: add equal check for admin_info.key
        if billboard_info.owner != program_id ||
            pool_info.owner_info != program_id ||
            !admin_info.is_signer{
            return Err(LotteryError::InvalidPermission.into());
        } 

        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        lottery.fund = fund;
        lottery.award = 0;
        lottery.price = price;
        lottery.fee =  *fee_info.key;
        lottery.billboard = *billboard_info.key;
        lottery.pool.clear();
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;
        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_signin(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
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
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let system_program_info= next_account_info(account_info_iter)?;
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
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        log_info(format!("accounts len:{}", accounts.len()).as_str());
        let account_info_iter = &mut accounts.iter();
        let pool_info= next_account_info(account_info_iter)?;
        let award_info = next_account_info(account_info_iter)?;
        let clock_sysvar_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(clock_sysvar_info)?;

        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        if lottery.pool.len() == 0 {
            return Ok(());
        }
        let mut total_lottery= 0u64;
        for (_, v) in &lottery.pool {
            total_lottery += *v as u64; 
        }
        let l:u64 = clock.epoch % total_lottery;
        log_info(&format!("l for winner is {}", l));
        let mut total_lottery = 0u64;
        let mut winner : Pubkey = Pubkey::new_from_array([0u8; 32]);
        for (k, v) in &lottery.pool {
            total_lottery += *v as u64; 
            if total_lottery == l {
                winner = *k;
                log_info(&format!("winner is {}", winner));
                break;
            }
        }
        
        if winner == Pubkey::new_from_array([0u8; 32]) {
            return Ok(());
        }

        let mut award= AwardState::unpack_unchecked(&award_info.data.borrow())?;
        let bill = AwardBill{
            account: winner,
            award: lottery.award + lottery.fund,
            rewarded: false,
        };
        award.billboard.push(bill);
        AwardState::pack(award, &mut award_info.data.borrow_mut())?;

        lottery.award = 0;
        lottery.pool.clear();
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;

        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_reward(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let system_program_info= next_account_info(account_info_iter)?;
        let account_info= next_account_info(account_info_iter)?;
        let award_info = next_account_info(account_info_iter)?;
        let program_info = next_account_info(account_info_iter)?;
        let mut award= AwardState::unpack_unchecked(&award_info.data.borrow())?;

        for val in &mut award.billboard {
            if ! val.rewarded  {
                // TODO send award
                // need not check balance Cau'z it will fail
                invoke(
                    &system_instruction::transfer(
                        program_info.key,
                        account_info.key,
                        val.award,
                    ),
                    &[
                        program_info.clone(),
                        account_info.clone(),
                        system_program_info.clone(),
                    ],
                )?;
                val.rewarded = true;
            }
        }
       
        AwardState::pack(award, &mut award_info.data.borrow_mut())?;
        Ok(())
    }
 
}
