//! Program state processor

use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{MAX_PLAYER, LOTTERY_STATE_LEN, AwardState, AwardBill},
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
use arrayref::{array_mut_ref, array_ref, mut_array_refs};


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

            LotteryInstruction::GM{
                fund,
                price
            } => {
                log_info("Instruction: GM");
                Self::process_gm(program_id, accounts, fund, price)
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
        if pool_info.data_len() != LOTTERY_STATE_LEN ||
            billboard_info.data_len() != AwardState::LEN{
            return Err(LotteryError::InvalidAccountLength.into());
        }

        let pool_data = &mut pool_info.data.borrow_mut();
        let pool_buf = array_mut_ref![pool_data, 0, 42];
        let (
            fund_buf,
            billboard_addr_buf,
            player_count_buf,
        ) = mut_array_refs![pool_buf, 8, 32, 2];
        *fund_buf = fund.to_le_bytes();
        billboard_addr_buf.copy_from_slice(billboard_info.key.as_ref());
        *player_count_buf = 0u16.to_le_bytes();

        let mut billboard= AwardState::unpack_unchecked(&billboard_info.data.borrow())?;
        billboard.billboard.clear();
        AwardState::pack(billboard, &mut billboard_info.data.borrow_mut())?;

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

        if pool_info.data_len() != LOTTERY_STATE_LEN {
            return Err(LotteryError::InvalidAccountLength.into());
        }

        let pool_data = &mut pool_info.data.borrow_mut();
        let pool_buf = array_mut_ref![pool_data, 0, LOTTERY_STATE_LEN];
        let (
            _fund_buf,
            _billboard_addr_buf,
            player_count_buf,
            players_buf,
        ) = mut_array_refs![pool_buf, 8, 32, 2, MAX_PLAYER*32];
        let player_count = u16::from_le_bytes(*player_count_buf);
        if player_count >= MAX_PLAYER as u16 {
            return Err(LotteryError::TooManyPlayers.into());  
        }
        for i in 0..player_count {
            let s = 32*i as usize;
            let e = 32*(i+1) as usize;
            let player = Pubkey::new(&players_buf[s..e]);
            if player == *account_info.key {
                return  Err(LotteryError::AlreadySignin.into());  
            }
        }
        *player_count_buf = (1u16+player_count).to_le_bytes(); 
        let s = (player_count*32) as usize;
        let player_buf = array_mut_ref![players_buf, s, 32];
        player_buf.copy_from_slice(account_info.key.as_ref());

        Ok(())
    }

    /// Processes an [Initialize](enum.Instruction.html).
    pub fn process_gm(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        fund:u64,
        _price:u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let admin_info= next_account_info(account_info_iter)?;
        let pool_info= next_account_info(account_info_iter)?;


        //check permission first
        if Pubkey::from_str(Self::ADMIN_KEY).unwrap() != *admin_info.key {
            return Err(LotteryError::InvalidPermission.into());
        }
        if pool_info.owner != program_id ||
            !admin_info.is_signer{
            return Err(LotteryError::InvalidPermission.into());
        } 

        // check account's data length
        if pool_info.data_len() != LOTTERY_STATE_LEN {
            return Err(LotteryError::InvalidAccountLength.into());
        }

        let pool_data = &mut pool_info.data.borrow_mut();
        let pool_buf = array_mut_ref![pool_data, 0, 42];
        let fund_buf = array_mut_ref![pool_buf,0, 8];
        *fund_buf = fund.to_le_bytes();
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
        if pool_info.data_len() != LOTTERY_STATE_LEN ||
            award_info.data_len() != AwardState::LEN{
            return Err(LotteryError::InvalidAccountLength.into());
        }

        let pool_data = &mut pool_info.data.borrow_mut();
        let pool_buf = array_mut_ref![pool_data, 0, LOTTERY_STATE_LEN];
        let (
            fund_buf,
            _billboard_addr_buf,
            player_count_buf,
            players_buf,
        ) = mut_array_refs![pool_buf, 8, 32, 2, MAX_PLAYER*32];
        let fund = u64::from_le_bytes(*fund_buf);
        let player_count = u16::from_le_bytes(*player_count_buf);

        if player_count == 0 {
            return Ok(());
        }

        //log_info(&format!("unix_timestamp is {}", clock.unix_timestamp));
        //log_info(&format!("player count is {}", player_count));
        let l:u64 = (clock.unix_timestamp as u64) % (player_count as u64);
        log_info(&format!("l for winner is {}", l));
        let s = (l*32) as usize;
        let winner = Pubkey::new(array_ref!(players_buf,s,32));

        log_info(&format!("winner is {}", winner));
        
        let mut award= AwardState::unpack_unchecked(&award_info.data.borrow())?;
        let bill = AwardBill{
            account: winner,
            award: fund,
            rewarded: false,
            timestamp:clock.unix_timestamp,
        };
        award.billboard.push(bill);
        AwardState::pack(award, &mut award_info.data.borrow_mut())?;
        *player_count_buf = 0u16.to_le_bytes();

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
                if val.account != *account_info.key {
                    continue;
                }
                // need not check balance Cau'z it will fail
                log_info(&format!("send award to {}", val.account));
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
