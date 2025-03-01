#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod tools;

pub use solana_program;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

pub use account_info::address::{
    account_info, wallet_info,
};
pub use wallet_info::program::{check_id, id, ID};


pub fn account_info(
    funding_address: &Pubkey,
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
) -> Instruction {
    let account_info =
        account_info(wallet_address, token_mint_address);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(*funding_address, true),
            AccountMeta::new(associated_account_address, false),
            AccountMeta::new_readonly(*wallet_address, false),
            AccountMeta::new_readonly(*token_mint_address, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: vec![],
    }
}
