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
