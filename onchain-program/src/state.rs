//! State transition types


use arrayref::{array_mut_ref, array_ref};
//use num_enum::TryFromPrimitive;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::collections::HashMap;

/// LotteryPool data.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LotteryPool {
    /// all accounts
    pub accounts: HashMap<Pubkey, u16>,
}

impl Sealed for LotteryPool {}
impl IsInitialized for LotteryPool {
    fn is_initialized(&self) -> bool {
        true
    }
}
impl Pack for LotteryPool {
    const LEN: usize = 2+1000*(32+2);
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let count_buf = array_ref![src, 0, 2];
        let count =  u16::from_le_bytes(*count_buf);
        let mut accounts = HashMap::new();
        for i in 0..count {
            let i = i as usize;
            let offset:usize = 2+i*(32+2) ;
            let key_buf = array_ref![src,offset, 32];
            let key = Pubkey::new_from_array(*key_buf);
            let lottery_buf = array_ref![src,offset+32, 2];
            let lottery = u16::from_le_bytes(*count_buf);
            accounts.insert(key, lottery);
        }

        Ok(LotteryPool {
            accounts,
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let count_buf = array_mut_ref![dst, 0, 2];
        let count:u16 = self.accounts.len() as u16;
        count_buf.copy_from_slice(&count.to_le_bytes());

        let mut i:usize=0;
        for (key, val) in self.accounts.iter() {
            let offset:usize = 2+i*(32+2);
            let key_buf = array_mut_ref![dst, offset, 32];
            key_buf.copy_from_slice(key.as_ref());
            let lottery_buf = array_mut_ref![dst, offset+32, 2];
            lottery_buf.copy_from_slice(&val.to_le_bytes());
            i += 1;
        }
    }
}
