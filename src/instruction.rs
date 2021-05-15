use crate::error::DemoError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use std::convert::TryInto;
use std::mem::size_of;
use solana_program::account_info::AccountInfo;

#[derive(Debug, PartialEq)]
pub enum DemoInstruction {
    /// Initialize the program
    ///
    /// 0. `[writable, signer]`
    /// 1. `[writable]`
    InitializeInstruction(u64),
}

impl DemoInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use DemoError::InvalidInstruction;

        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let amount = Self::unpack_amount(rest)?;
                Self::InitializeInstruction(amount)
            },
            _ => return Err(DemoError::InvalidInstruction.into()),
        })
    }

    pub fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(DemoError::InvalidInstruction)?;

        Ok(amount)
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());

        match self {
            &Self::InitializeInstruction(amount) => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());

            },
        };
        buf
    }
}


