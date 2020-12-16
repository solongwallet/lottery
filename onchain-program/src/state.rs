//! State transition types


use arrayref::{array_mut_ref, array_ref};
//use num_enum::TryFromPrimitive;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// LotteryRecord 
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LotteryRecord{
    /// lottery's account
    pub account: Pubkey,
    /// amount for the lottery 
    pub amount: u16,
}

/// LotteryState data.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LotteryState {
    /// award for lottery
    pub award : u64,
    /// fund from solong
    pub fund : u64,
    /// price for a lottery
    pub price: u64,
    /// fee account
    pub fee : Pubkey,
    /// award accounts
    pub billboard: Pubkey,
    /// all accounts
    pub pool: Vec<LotteryRecord>,
}

impl Sealed for LotteryState {}
impl IsInitialized for LotteryState {
    fn is_initialized(&self) -> bool {
        true
    }
}
impl Pack for LotteryState {
    const LEN: usize = 8+8+8+32+32+2+1000*(32+2);
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let award_buf = array_ref![src, 0, 8];
        let award =  u64::from_le_bytes(*award_buf);
        let fund_buf = array_ref![src, 8, 8];
        let fund =  u64::from_le_bytes(*fund_buf);
        let price_buf = array_ref![src, 16, 8];
        let price =  u64::from_le_bytes(*price_buf);
        let fee_buf = array_ref![src, 24, 32];
        let fee = Pubkey::new_from_array(*fee_buf);
        let billboard_buf = array_ref![src, 56, 32];
        let billboard = Pubkey::new_from_array(*billboard_buf);
        let count_buf = array_ref![src, 88, 2];
        let count =  u16::from_le_bytes(*count_buf);
        let mut pool = Vec::new();
        for i in 0..count {
            let i = i as usize;
            let offset:usize = 90+i*(32+2) ;
            let account_buf = array_ref![src,offset, 32];
            let account= Pubkey::new_from_array(*account_buf);
            let amount_buf = array_ref![src,offset+32, 2];
            let amount = u16::from_le_bytes(*amount_buf);
            pool.push(LotteryRecord{
                account,
                amount,
            });
        }

        Ok(LotteryState {
            award,
            fund,
            price,
            fee,
            billboard,
            pool,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let award_buf = array_mut_ref![dst, 0, 8];
        award_buf.copy_from_slice(&self.award.to_le_bytes());
        let fund_buf = array_mut_ref![dst, 8, 8];
        fund_buf.copy_from_slice(&self.fund.to_le_bytes());
        let price_buf = array_mut_ref![dst, 16, 8];
        price_buf.copy_from_slice(&self.price.to_le_bytes());
        let fee_buf = array_mut_ref![dst, 24, 32];
        fee_buf.copy_from_slice(self.fee.as_ref());
        let billboard_buf = array_mut_ref![dst, 56, 32];
        billboard_buf.copy_from_slice(self.billboard.as_ref());
        let count_buf = array_mut_ref![dst, 88, 2];
        let count:u16 = self.pool.len() as u16;
        count_buf.copy_from_slice(&count.to_le_bytes());
        let mut i:usize=0;
        for val in &self.pool{
            let offset:usize = 90+i*(32+2);
            let key_buf = array_mut_ref![dst, offset, 32];
            key_buf.copy_from_slice(val.account.as_ref());
            let lottery_buf = array_mut_ref![dst, offset+32, 2];
            lottery_buf.copy_from_slice(&val.amount.to_le_bytes());
            i += 1;
        }
    }
}


/// AwardBill
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AwardBill{
    /// winner's account
    pub account: Pubkey,
    /// award for the winner
    pub award: u64,
    /// if winner has rewarded
    pub rewarded: bool,
}

/// AwardState data.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AwardState {
    /// all winner billboard
    pub billboard: Vec<AwardBill>,
}

impl Sealed for AwardState {}
impl IsInitialized for AwardState {
    fn is_initialized(&self) -> bool {
        true
    }
}
impl Pack for AwardState {
    const LEN: usize = 1000*(32+9);
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut billboard = Vec::new();
        let count_buf = array_ref![src, 0, 2];
        let count =  u16::from_le_bytes(*count_buf);
        for i in 0..count {
            let i = i as usize;
            let offset:usize = 2+i*(32+9) ;
            let account_buf = array_ref![src,offset, 32];
            let account= Pubkey::new_from_array(*account_buf);
            let award_buf= array_ref![src,offset+32, 8];
            let award = u64::from_le_bytes(*award_buf);
            let rewarded_buf = array_ref![src,offset+40, 1];
            let rewarded = rewarded_buf[0] != 0;
            billboard.push(AwardBill{
                account,
                award,
                rewarded
            });
        }

        Ok(AwardState {
            billboard,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let count_buf = array_mut_ref![dst, 0, 2];
        let count:u16 = self.billboard.len() as u16;
        count_buf.copy_from_slice(&count.to_le_bytes());
        let mut i:usize=0;
        for val in &self.billboard{
            let offset:usize = 2+i*(32+9);
            let account_buf = array_mut_ref![dst, offset, 32];
            account_buf.copy_from_slice(val.account.as_ref());
            let award_buf = array_mut_ref![dst, offset+32, 8];
            award_buf.copy_from_slice(&val.award.to_le_bytes());
            let reward_buf = array_mut_ref![dst, offset+40, 1];
            if val.rewarded {
                reward_buf[0] = 1;
            } else {
                reward_buf[0] = 0;
            }
            i += 1;
        }
    }
}
