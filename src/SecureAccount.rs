use {
    secure_account::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program::{get_return_data, invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
    },
    secure_account::extension::ExtensionType,
    std::convert::TryInto,
};

pub fn create_secure_account<'a>(
    payer: &AccountInfo<'a>,
    rent: &Rent,
    space: usize,
    owner: &Pubkey,
    system_program: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    new_account_signer_seeds: &[&[u8]],
) -> ProgramResult {
    if new_account.lamports() > 0 {
        let required_lamports = rent
            .minimum_balance(space)
            .max(1)
            .saturating_sub(new_account.lamports());

        if required_lamports > 0 {
            invoke(
                &system_instruction::transfer(payer.key, new_account.key, required_lamports),
                &[
                    payer.clone(),
                    new_account.clone(),
                    system_program.clone(),
                ],
            )?;
        }

        invoke_signed(
            &system_instruction::allocate(new_account.key, space as u64),
            &[new_account.clone(), system_program.clone()],
            &[new_account_signer_seeds],
        )?;

        invoke_signed(
            &system_instruction::assign(new_account.key, owner),
            &[new_account.clone(), system_program.clone()],
            &[new_account_signer_seeds],
        )
    } else {
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                new_account.key,
                rent.minimum_balance(space).max(1),
                space as u64,
                owner,
            ),
            &[
                payer.clone(),
                new_account.clone(),
                system_program.clone(),
            ],
            &[new_account_signer_seeds],
        )
    }
}


pub fn get_account_len<'a>(
    mint: &AccountInfo<'a>,
    secure_account: &AccountInfo<'a>,
    extension_types: &[ExtensionType],
) -> Result<usize, ProgramError> {
    invoke(
        &secure_account::instruction::get_account_data_size(
            secure_account.key,
            mint.key,
            extension_types,
        )?,
        &[mint.clone(), secure_account.clone()],
    )?;
    get_return_data()
        .ok_or(ProgramError::InvalidInstructionData)
        .and_then(|(key, data)| {
            if key != *secure_account.key {
                return Err(ProgramError::IncorrectProgramId);
            }
            data.try_into()
                .map(usize::from_le_bytes)
                .map_err(|_| ProgramError::InvalidInstructionData)
        })
}
