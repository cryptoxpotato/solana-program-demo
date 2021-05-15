//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the demo program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum DemoError {
    // 0.
    /// Invalid instruction used
    #[error("Invalid Instruction")]
    InvalidInstruction,
}
impl From<DemoError> for ProgramError {
    fn from(e: DemoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for DemoError {
    fn type_of() -> &'static str {
        "Demo Error"
    }
}
