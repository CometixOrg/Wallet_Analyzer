use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum WalletInfoError {
    #[error("Invalid owner")]
    InvalidOwner,
}

impl From<WalletInfoError> for ProgramError {
    fn from(e: WalletInfoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
