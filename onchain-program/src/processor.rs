//! Program state processor

use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{LotteryState, LotteryRecord, AwardState, AwardBill},
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
use std::str::FromStr;


/// Program state handler.
pub struct Processor {}
impl Processor {
    const ADMIN_KEY: &'static str = "4n3CDb6jtrbsChMVUSbnARknv1S6wCTN1bRWqopfH35B";
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
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
        log_info(format!("process_initialize fund:{} price:{}", fund, price ).as_str());
        let account_info_iter = &mut accounts.iter();
        let admin_info= next_account_info(account_info_iter)?;
        let fee_info= next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;
        let billboard_info= next_account_info(account_info_iter)?;

        //check permission first
        if Pubkey::from_str(Self::ADMIN_KEY).unwrap() != *admin_info.key {
            return Err(LotteryError::InvalidPermission.into());
        }
        if billboard_info.owner != program_id ||
            pool_info.owner != program_id ||
            !admin_info.is_signer{
            return Err(LotteryError::InvalidPermission.into());
        } 

        // check account's data length
        if pool_info.data_len() != LotteryState::LEN ||
            billboard_info.data_len() != AwardState::LEN{
            return Err(LotteryError::InvalidAccountLength.into());
        }

        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        lottery.fund = fund;
        lottery.award = 0;
        lottery.price = price;
        lottery.fee =  *fee_info.key;
        lottery.billboard = *billboard_info.key;
        lottery.pool.clear();
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;

        let mut award= AwardState::unpack_unchecked(&billboard_info.data.borrow())?;
        award.billboard.clear();
        AwardState::pack(award, &mut billboard_info.data.borrow_mut())?;

        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_signin(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let account_info= next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;

        if pool_info.data_len() != LotteryState::LEN {
            return Err(LotteryError::InvalidAccountLength.into());
        }
    
        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        log_info(&format!("process_signin lottery:award[{}] fund[{}] price[{}] fee[{}] billboard[{}] pool[{}]",
                lottery.award, lottery.fund, lottery.price, lottery.fee, lottery.billboard, lottery.pool.len()));
        let mut founded = false;
        for val in &mut lottery.pool{
            if val.account == *account_info.key {
                val.amount += 1;
                log_info(&format!("found account:{} lottery:{}", val.account, val.amount));
                founded = true;
                if val.signin {
                    return Err(LotteryError::AlreadySignin.into());
                }
                break;
            }
        }
        if ! founded {
            lottery.pool.push(LotteryRecord{
                account: *account_info.key,
                amount: 1,
                signin: true,
            });
        }
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;
        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_buy(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let system_program_info= next_account_info(account_info_iter)?;
        let account_info= next_account_info(account_info_iter)?;
        let admin_info= next_account_info(account_info_iter)?;
        let fee_info= next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;

        // check pool data length first 
        // check fee then 
        if pool_info.data_len() != LotteryState::LEN {
            return Err(LotteryError::InvalidAccountLength.into());
        }
        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        if *fee_info.key != lottery.fee {
            return Err(LotteryError::InvaliedFee.into());
        }

        // check balance
        if account_info.lamports() < 1000_000_000 {
            return Err(LotteryError::LowBalance.into());
        }

        let price_lamports = 1000_000_000;
        // send to admin
        invoke(
            &system_instruction::transfer(
                account_info.key,
                admin_info.key,
                price_lamports*9/10,
            ),
            &[
                account_info.clone(),
                admin_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        // send to fee
        invoke(
            &system_instruction::transfer(
                account_info.key,
                fee_info.key,
                price_lamports*1/10,
            ),
            &[
                account_info.clone(),
                fee_info.clone(),
                system_program_info.clone(),
            ],
        )?;


        lottery.award +=price_lamports*9/10;
        let mut founded = false;
        for val in &mut lottery.pool{
            if val.account == *account_info.key {
                val.amount += 1;
                founded = true;
                break;
            }
        }
        if ! founded {
            lottery.pool.push(LotteryRecord{
                account: *account_info.key,
                amount: 1,
                signin: false,
            });
        }
        LotteryState::pack(lottery, &mut pool_info.data.borrow_mut())?;
        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_roll(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let clock_sysvar_info = next_account_info(account_info_iter)?;
        let admin_info = next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;
        let award_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(clock_sysvar_info)?;
        
        //check permission first
        if Pubkey::from_str(Self::ADMIN_KEY).unwrap() != *admin_info.key {
            return Err(LotteryError::InvalidPermission.into());
        }
        if award_info.owner != program_id ||
            pool_info.owner != program_id ||
            !admin_info.is_signer{
            return Err(LotteryError::InvalidPermission.into());
        } 

        // check account's data length
        if pool_info.data_len() != LotteryState::LEN ||
            award_info.data_len() != AwardState::LEN{
            return Err(LotteryError::InvalidAccountLength.into());
        }


        let mut lottery= LotteryState::unpack_unchecked(&pool_info.data.borrow())?;
        if lottery.pool.len() == 0 {
            return Ok(());
        }
        let mut total_lottery= 0u64;
        for val in &lottery.pool{
            total_lottery += val.amount as u64;
        }
        let l:u64 = clock.epoch % total_lottery  +1 ;
        log_info(&format!("l for winner is {}", l));
        let mut total_lottery = 0u64;
        let mut winner : Pubkey = Pubkey::new_from_array([0u8; 32]);
        for val in &lottery.pool{
            total_lottery += val.amount as u64;
            if total_lottery >= l {
                winner = val.account;
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
            timestamp:clock.unix_timestamp,
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
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let system_program_info= next_account_info(account_info_iter)?;
        let admin_info = next_account_info(account_info_iter)?;
        let account_info= next_account_info(account_info_iter)?;
        let award_info = next_account_info(account_info_iter)?;

        //check permission first
        if Pubkey::from_str(Self::ADMIN_KEY).unwrap() != *admin_info.key {
            return Err(LotteryError::InvalidPermission.into());
        }
        if award_info.owner != program_id ||
            !admin_info.is_signer{
            return Err(LotteryError::InvalidPermission.into());
        } 

        // check account's data length
        if award_info.data_len() != AwardState::LEN{
            return Err(LotteryError::InvalidAccountLength.into());
        }
        
        let mut award= AwardState::unpack_unchecked(&award_info.data.borrow())?;


        for val in &mut award.billboard {
            log_info(&format!("come to send award: {}:{}:{}", val.account, val.award, val.rewarded));
            if ! val.rewarded  {
                // need not check balance Cau'z it will fail
                log_info("before send award");
                invoke(
                    &system_instruction::transfer(
                        admin_info.key,
                        account_info.key,
                        val.award,
                    ),
                    &[
                        admin_info.clone(),
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
