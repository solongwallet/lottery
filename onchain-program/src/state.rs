//! State transition types


use arrayref::{array_mut_ref, array_ref};
//use num_enum::TryFromPrimitive;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    clock::UnixTimestamp,
};

/// LotteryRecord 
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LotteryRecord{
    /// lottery's account
    pub account: Pubkey,
    /// amount for the lottery 
    pub amount: u16,
    /// is signin
    pub signin:bool,
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
    const LEN: usize = 8+8+8+32+32+2+1000*(32+2+1);
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
            let offset:usize = 90+i*(32+2+1) ;
            let account_buf = array_ref![src,offset, 32];
            let account= Pubkey::new_from_array(*account_buf);
            let amount_buf = array_ref![src,offset+32, 2];
            let amount = u16::from_le_bytes(*amount_buf);
            let signin_buf = array_ref![src,offset+34, 1];
            let mut signin = true;
            if signin_buf[0] == 0 {
                signin = false;
            }
            pool.push(LotteryRecord{
                account,
                amount,
                signin,
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
            let offset:usize = 90+i*(32+2+1);
            let key_buf = array_mut_ref![dst, offset, 32];
            key_buf.copy_from_slice(val.account.as_ref());
            let lottery_buf = array_mut_ref![dst, offset+32, 2];
            lottery_buf.copy_from_slice(&val.amount.to_le_bytes());
            let signin_buf = array_mut_ref![dst, offset+34, 1];
            if val.signin {
                signin_buf[0] = 1;
            } else {
                signin_buf[0] = 0;
            }
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
    /// timestamp for this
    pub timestamp:UnixTimestamp,
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
    const LEN: usize = 2+1000*(32+8+1+8);
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut billboard = Vec::new();
        let count_buf = array_ref![src, 0, 2];
        let count =  u16::from_le_bytes(*count_buf);
        for i in 0..count {
            let i = i as usize;
            let offset:usize = 2+i*(32+8+1+8) ;
            let account_buf = array_ref![src,offset, 32];
            let account= Pubkey::new_from_array(*account_buf);
            let award_buf= array_ref![src,offset+32, 8];
            let award = u64::from_le_bytes(*award_buf);
            let rewarded_buf = array_ref![src,offset+40, 1];
            let rewarded = rewarded_buf[0] != 0;
            let timestamp_buf= array_ref![src,offset+41, 8];
            let timestamp= UnixTimestamp::from_le_bytes(*timestamp_buf);
            billboard.push(AwardBill{
                account,
                award,
                rewarded,
                timestamp,
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
            let offset:usize = 2+i*(32+8+1+8);
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
            let timestamp_buf = array_mut_ref![dst, offset+41, 8];
            timestamp_buf.copy_from_slice(&val.timestamp.to_le_bytes());
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_lottery_state() {
        let check = LotteryState{
            award:0u64,
            fund:0u64,
            price:0u64,
            fee:Pubkey::new(&[0u8;32]),
            billboard:Pubkey::new(&[0u8;32]),
            pool: Vec::new(),
        };
        let mut packed = [0u8;LotteryState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[0u8,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0u8,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0u8,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0u8;32]);
        expect.extend_from_slice(&[0u8;32]);
        expect.extend_from_slice(&[0u8;2]);
        expect.extend_from_slice(&[0u8;LotteryState::LEN-(8+8+8+32+32+2)]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = LotteryState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check); 


        let check = LotteryState{
            award:0u64,
            fund:10_000_000_000u64,
            price:1_000_000_000u64,
            fee:Pubkey::new(&[1u8;32]),
            billboard:Pubkey::new(&[2u8;32]),
            pool: Vec::new(),
        };
        let mut packed = [0u8;LotteryState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[0u8,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[0, 202, 154, 59, 0, 0, 0, 0]);
        expect.extend_from_slice(&[1u8;32]);
        expect.extend_from_slice(&[2u8;32]);
        expect.extend_from_slice(&[0u8;2]);
        expect.extend_from_slice(&[0u8;LotteryState::LEN-(8+8+8+32+32+2)]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = LotteryState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check); 

        let mut pool = Vec::new();
        let p = LotteryRecord {
            account: Pubkey::new(&[3u8;32]),
            amount:2u16,
            signin:true,
        };
        pool.push(p);
        let check = LotteryState{
            award:0u64,
            fund:10_000_000_000u64,
            price:1_000_000_000u64,
            fee:Pubkey::new(&[1u8;32]),
            billboard:Pubkey::new(&[2u8;32]),
            pool, 
        };
        let mut packed = [0u8;LotteryState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[0u8,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[0, 202, 154, 59, 0, 0, 0, 0]);
        expect.extend_from_slice(&[1u8;32]);
        expect.extend_from_slice(&[2u8;32]);
        expect.extend_from_slice(&[1, 0]);
        expect.extend_from_slice(&[3; 32]);
        expect.extend_from_slice(&[2, 0]);
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[0u8;LotteryState::LEN-(8+8+8+32+32+2)-35]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = LotteryState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check); 



        let mut pool = Vec::new();
        let p = LotteryRecord {
            account: Pubkey::new(&[3u8;32]),
            amount:2u16,
            signin:true,
        };
        pool.push(p);
        let p = LotteryRecord {
            account: Pubkey::new(&[3u8;32]),
            amount:2u16,
            signin:false,
        };
        pool.push(p);
        let check = LotteryState{
            award:0u64,
            fund:10_000_000_000u64,
            price:1_000_000_000u64,
            fee:Pubkey::new(&[1u8;32]),
            billboard:Pubkey::new(&[2u8;32]),
            pool, 
        };
        let mut packed = [0u8;LotteryState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[0u8,0,0,0,0,0,0,0]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[0, 202, 154, 59, 0, 0, 0, 0]);
        expect.extend_from_slice(&[1u8;32]);
        expect.extend_from_slice(&[2u8;32]);
        expect.extend_from_slice(&[2, 0]);
        expect.extend_from_slice(&[3; 32]);
        expect.extend_from_slice(&[2, 0]);
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[3; 32]);
        expect.extend_from_slice(&[2, 0]);
        expect.extend_from_slice(&[0]);
        expect.extend_from_slice(&[0u8;LotteryState::LEN-(8+8+8+32+32+2)-35*2]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = LotteryState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check);
    }

    #[test]
    fn test_instruction_award_state() {
        let check = AwardState{
            billboard: Vec::new(),
        };
        let mut packed = [0u8;AwardState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[0u8;2]);
        expect.extend_from_slice(&[0u8;AwardState::LEN-(2)]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = AwardState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check);
        
        

        let mut billboard = Vec::new();
        let b = AwardBill {
            account: Pubkey::new(&[0u8;32]),
            award:0u64,
            rewarded:false,
            timestamp:0,
        };
        billboard.push(b);
        let check = AwardState{
            billboard,
        };
        let mut packed = [0u8;AwardState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[1u8,0]);
        expect.extend_from_slice(&[0;32]);
        expect.extend_from_slice(&[0;8]);
        expect.extend_from_slice(&[0]);
        expect.extend_from_slice(&[0;8]);
        expect.extend_from_slice(&[0u8;AwardState::LEN-(2+49)]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = AwardState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check); 

        let mut billboard = Vec::new();
        let b = AwardBill {
            account: Pubkey::new(&[0u8;32]),
            award:0u64,
            rewarded:false,
            timestamp: 1608273769,
        };
        billboard.push(b);
        let b = AwardBill {
            account: Pubkey::new(&[1u8;32]),
            award:10_000_000_000u64,
            rewarded:true,
            timestamp: 1608273769,
        };
        billboard.push(b);


        let check = AwardState{
            billboard,
        };
        let mut packed = [0u8;AwardState::LEN];
        check.pack_into_slice(&mut packed);
        let mut expect = Vec::new();
        expect.extend_from_slice(&[2u8,0]);
        expect.extend_from_slice(&[0;32]);
        expect.extend_from_slice(&[0;8]);
        expect.extend_from_slice(&[0]);
        expect.extend_from_slice(&[105, 79, 220, 95, 0, 0, 0, 0]);
        expect.extend_from_slice(&[1;32]);
        expect.extend_from_slice(&[0, 228, 11, 84, 2, 0, 0, 0]);
        expect.extend_from_slice(&[1]);
        expect.extend_from_slice(&[105, 79, 220, 95, 0, 0, 0, 0]);
        expect.extend_from_slice(&[0u8;AwardState::LEN-(2+49*2)]);
        assert_eq!(packed.to_vec(), expect);
        let unpacked = AwardState::unpack_from_slice(&expect).unwrap();
        assert_eq!(unpacked, check); 

    }

}
