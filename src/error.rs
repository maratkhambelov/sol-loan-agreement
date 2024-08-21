use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoanContractError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Input data exceeds max length")]
    InvalidDataLength,

    #[error("Parsing error from data item")]
    InvalidItemData,

    #[error("Item owner is not matching")]
    InvalidOwnerItem,

    #[error("Contract is already active")]
    UnavailableSignActiveContract

}

impl From<LoanContractError> for ProgramError {
    fn from(e: LoanContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
