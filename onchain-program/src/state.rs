//! State transition types


use arrayref::{array_mut_ref, array_ref};
//use num_enum::TryFromPrimitive;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::collections::HashMap;

/// LotteryState data.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LotteryState {
    /// price for a lottery
    pub price: u64,
    /// fee account
    pub fee : Pubkey,
    /// all accounts
    pub pool: HashMap<Pubkey, u16>,
}

impl Sealed for LotteryState {}
impl IsInitialized for LotteryState {
    fn is_initialized(&self) -> bool {
        true
    }
}
impl Pack for LotteryState {
    const LEN: usize = 8+32+2+1000*(32+2);
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let price_buf = array_ref![src, 0, 8];
        let price =  u64::from_le_bytes(*price_buf);
        let fee_buf = array_ref![src, 8, 32];
        let fee = Pubkey::new_from_array(*fee_buf);
        let count_buf = array_ref![src, 40, 2];
        let count =  u16::from_le_bytes(*count_buf);
        let mut pool = HashMap::new();
        for i in 0..count {
            let i = i as usize;
            let offset:usize = 42+i*(32+2) ;
            let key_buf = array_ref![src,offset, 32];
            let key = Pubkey::new_from_array(*key_buf);
            let lottery_buf = array_ref![src,offset+32, 2];
            let lottery = u16::from_le_bytes(*count_buf);
            pool.insert(key, lottery);
        }

        Ok(LotteryState {
            price,
            fee,
            pool,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let price_buf = array_mut_ref![dst, 0, 8];
        price_buf.copy_from_slice(&self.price.to_le_bytes());
        let fee_buf = array_mut_ref![dst, 8, 32];
        fee_buf.copy_from_slice(self.fee.as_ref());
        let count_buf = array_mut_ref![dst, 40, 2];
        let count:u16 = self.pool.len() as u16;
        count_buf.copy_from_slice(&count.to_le_bytes());
        let mut i:usize=0;
        for (key, val) in self.pool.iter() {
            let offset:usize = 42+i*(32+2);
            let key_buf = array_mut_ref![dst, offset, 32];
            key_buf.copy_from_slice(key.as_ref());
            let lottery_buf = array_mut_ref![dst, offset+32, 2];
            lottery_buf.copy_from_slice(&val.to_le_bytes());
            i += 1;
        }
    }
}
