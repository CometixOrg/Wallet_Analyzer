use {
    num_derive::FromPrimitive,
    solana_program::{decode_error::DecodeError, program_error::ProgramError},
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AccountError {
    InvalidOwner,
}
impl From<AccountError> for ProgramError {
    fn from(e: AssociatedTokenAccountError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for AccountError {
    fn type_of() -> &'static str {
        "AccountError"
    }
}
