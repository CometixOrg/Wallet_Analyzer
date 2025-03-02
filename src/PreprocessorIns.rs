use {
    crate::{
        checks::{validate_accounts, get_amount_and_decimals},
        errors::WalletInfoError,
        utils::{get_wallet_info_signer_seeds},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke_signed,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    wallet_info::instruction,
};

pub fn process_recover_nested(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let nested_wallet_info_account_info = next_account_info(account_info_iter)?;
    let nested_wallet_info_mint_info = next_account_info(account_info_iter)?;
    let destination_wallet_info_account_info = next_account_info(account_info_iter)?;
    let owner_wallet_info_account_info = next_account_info(account_info_iter)?;
    let owner_wallet_info_mint_info = next_account_info(account_info_iter)?;
    let wallet_account_info = next_account_info(account_info_iter)?;
    let wallet_info_program_info = next_account_info(account_info_iter)?;
    let wallet_info_program_id = wallet_info_program_info.key;

    validate_accounts(
        wallet_info_program_id,
        nested_wallet_info_account_info,
        nested_wallet_info_mint_info,
        destination_wallet_info_account_info,
        owner_wallet_info_account_info,
        owner_wallet_info_mint_info,
        wallet_account_info,
    )?;

    let (amount, decimals) = get_amount_and_decimals(
        wallet_info_program_id,
        owner_wallet_info_account_info,
        nested_wallet_info_account_info,
        nested_wallet_info_mint_info,
        wallet_account_info,
    )?;

    let wallet_info_signer_seeds = get_wallet_info_signer_seeds(
        wallet_account_info.key,
        wallet_info_program_id,
        owner_wallet_info_mint_info.key,
    );

    invoke_signed(
        &instruction::transfer_checked(
            wallet_info_program_id,
            nested_wallet_info_account_info.key,
            nested_wallet_info_mint_info.key,
            destination_wallet_info_account_info.key,
            owner_wallet_info_account_info.key,
            &[],
            amount,
            decimals,
        )?,
        &[
            nested_wallet_info_account_info.clone(),
            nested_wallet_info_mint_info.clone(),
            destination_wallet_info_account_info.clone(),
            owner_wallet_info_account_info.clone(),
            wallet_info_program_info.clone(),
        ],
        &[&wallet_info_signer_seeds],
    )?;

    invoke_signed(
        &instruction::close_account(
            wallet_info_program_id,
            nested_wallet_info_account_info.key,
            wallet_account_info.key,
            owner_wallet_info_account_info.key,
            &[],
        )?,
        &[
            nested_wallet_info_account_info.clone(),
            wallet_account_info.clone(),
            owner_wallet_info_account_info.clone(),
            wallet_info_program_info.clone(),
        ],
        &[&wallet_info_signer_seeds],
    )?;

    Ok(())
}
