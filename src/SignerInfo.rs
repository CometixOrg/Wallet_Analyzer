use solana_program::pubkey::Pubkey;

pub fn get_wallet_info_signer_seeds<'a>(
    wallet_key: &Pubkey,
    program_id: &Pubkey,
    mint_key: &Pubkey,
) -> [&'a [u8]; 4] {
    [
        &wallet_key.to_bytes(),
        &program_id.to_bytes(),
        &mint_key.to_bytes(),
        &[0],
    ]
}
