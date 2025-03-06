use wallet_program_test::*;
use wallet_sdk::{
    account::Account as WalletAccount,
    signature::Keypair,
    transaction::{Transaction, TransactionError},
};

pub fn program_wallet_2025(wallet_mint_address: Pubkey) -> ProgramTest {
    let mut program_test = ProgramTest::new(
        "wallet_program",
        wallet_mint_address,
        processor!(wallet_program::entry),
    );
    program_test.add_account(
        wallet_mint_address,
        WalletAccount::new(1_000_000_000, Account::LEN, &spl_wallet_2025::id()),
    );
    program_test
}
