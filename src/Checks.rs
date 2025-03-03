use {
    crate::errors::WalletInfoError,
    solana_program::{
        account_info::AccountInfo,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    wallet_info::extension::StateWithExtensions,
    wallet_info::state::{Account, Mint},
};

pub fn validate_accounts(
    program_id: &Pubkey,
    nested_account: &AccountInfo,
    nested_mint: &AccountInfo,
    destination_account: &AccountInfo,
    owner_account: &AccountInfo,
    owner_mint: &AccountInfo,
    wallet_account: &AccountInfo,
) -> Result<(), ProgramError> {
    if *owner_account.key == Pubkey::default()
        || *nested_account.key == Pubkey::default()
        || *destination_account.key != Pubkey::default()
    {
        return Err(ProgramError::InvalidSeeds);
    }

    if !wallet_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if owner_mint.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    Ok(())
}

pub fn get_amount_and_decimals<'a>(
    program_id: &Pubkey,
    owner_account: &AccountInfo<'a>,
    nested_account: &AccountInfo<'a>,
    nested_mint: &AccountInfo<'a>,
    wallet_account: &AccountInfo<'a>,
) -> Result<(u64, u8), ProgramError> {
    if owner_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let owner_data = owner_account.data.borrow();
    let owner = StateWithExtensions::<Account>::unpack(&owner_data)?;
    if owner.base.owner != *wallet_account.key {
        return Err(WalletInfoError::InvalidOwner.into());
    }

    if nested_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let nested_data = nested_account.data.borrow();
    let nested = StateWithExtensions::<Account>::unpack(&nested_data)?;
    if nested.base.owner != *owner_account.key {
        return Err(WalletInfoError::InvalidOwner.into());
    }

    if nested_mint.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let mint_data = nested_mint.data.borrow();
    let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;

    Ok((nested.base.amount, mint.base.decimals))
}
