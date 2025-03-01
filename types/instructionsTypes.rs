use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
pub use wallet::instruction::*;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum InstructionTypes {
    Check,
    Decline,
    Accept,
}
