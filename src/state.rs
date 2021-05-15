use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use enum_dispatch::enum_dispatch;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use std::convert::TryInto;
use crate::error::DemoError;

pub struct demoState {
    pub amount:u64,
}

impl Sealed for demoState{}

impl Pack for demoState {
    const LEN: usize = 8;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let amount =  src
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(DemoError::InvalidInstruction)?;

        Ok(demoState{amount})
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 8];
        let amount = self.amount;
        *dst = amount.to_le_bytes();
    }
}

