use std::convert::TryInto;
use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use crate::instruction::DemoInstruction;

/// Program state handler.
pub struct Processor {}

impl Processor {
    pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = DemoInstruction::unpack(instruction_data)?;
        match instruction {
            DemoInstruction::InitializeInstruction(amount) => {
                msg!("Instruction: InitializeMint");
            }
        };
        Ok(())
    }
}