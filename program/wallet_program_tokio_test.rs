use {
    program_wallet::program_wallet_2025,
    wallet_program::{instruction::*, pubkey::Pubkey},
    wallet_program_test::*,
    wallet_sdk::{
        account::Account as WalletAccount,
        program_option::COption,
        program_pack::Pack,
        signature::Signer,
        signer::keypair::Keypair,
        system_instruction::create_account,
        transaction::{Transaction, TransactionError},
    },
    spl_associated_wallet_account::{
        error::AssociatedWalletAccountError,
        instruction::{
            create_associated_wallet_account, create_associated_wallet_account_idempotent,
        },
    },
    spl_associated_wallet_account_client::address::get_associated_wallet_address_with_program_id,
    spl_wallet_2025::{
        extension::ExtensionType,
        instruction::initialize_account,
        state::{Account, AccountState},
    },
};

#[tokio::test]
async fn success_wallet_exists() {
    let wallet_address = Pubkey::new_unique();
    let wallet_mint_address = Pubkey::new_unique();
    let associated_wallet_address = get_associated_wallet_address_with_program_id(
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );

    let (mut banks_client, payer, recent_blockhash) =
        program_wallet_2025(wallet_mint_address).start().await;
    let rent = banks_client.get_rent().await.unwrap();
    let expected_wallet_account_len =
        ExtensionType::try_calculate_account_len::<Account>(&[ExtensionType::ImmutableOwner])
            .unwrap();
    let expected_wallet_account_balance = rent.minimum_balance(expected_wallet_account_len);

    let instruction = create_associated_wallet_account_idempotent(
        &payer.pubkey(),
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let associated_account = banks_client
        .get_account(associated_wallet_address)
        .await
        .expect("get_account")
        .expect("associated_account not none");
    assert_eq!(associated_account.data.len(), expected_wallet_account_len);
    assert_eq!(associated_account.owner, spl_wallet_2025::id());
    assert_eq!(associated_account.lamports, expected_wallet_account_balance);

    let instruction = create_associated_wallet_account(
        &payer.pubkey(),
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    assert_eq!(
        banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(0, InstructionError::IllegalOwner)
    );

    let recent_blockhash = banks_client
        .get_new_latest_blockhash(&recent_blockhash)
        .await
        .unwrap();

    let instruction = create_associated_wallet_account_idempotent(
        &payer.pubkey(),
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let associated_account = banks_client
        .get_account(associated_wallet_address)
        .await
        .expect("get_account");
    assert_eq!(associated_account.data.len(), expected_wallet_account_len);
    assert_eq!(associated_account.owner, spl_wallet_2025::id());
    assert_eq!(associated_account.lamports, expected_wallet_account_balance);
}

#[tokio::test]
async fn fail_wallet_exists_with_wrong_owner() {
    let wallet_address = Pubkey::new_unique();
    let wallet_mint_address = Pubkey::new_unique();
    let associated_wallet_address = get_associated_wallet_address_with_program_id(
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );

    let wrong_owner = Pubkey::new_unique();
    let mut associated_wallet_account =
        WalletAccount::new(1_000_000_000, Account::LEN, &spl_wallet_2025::id());
    let wallet_account = Account {
        mint: wallet_mint_address,
        owner: wrong_owner,
        amount: 0,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    Account::pack(wallet_account, &mut associated_wallet_account.data).unwrap();
    let mut pt = program_wallet_2025(wallet_mint_address);
    pt.add_account(associated_wallet_address, associated_wallet_account);
    let (banks_client, payer, recent_blockhash) = pt.start().await;

    let instruction = create_associated_wallet_account_idempotent(
        &payer.pubkey(),
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    assert_eq!(
        banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(AssociatedWalletAccountError::InvalidOwner as u32)
        )
    );
}

#[tokio::test]
async fn fail_non_awa() {
    let wallet_mint_address = Pubkey::new_unique();
    let (banks_client, payer, recent_blockhash) =
        program_wallet_2025(wallet_mint_address).start().await;

    let rent = banks_client.get_rent().await.unwrap();
    let wallet_account_len =
        ExtensionType::try_calculate_account_len::<Account>(&[ExtensionType::ImmutableOwner])
            .unwrap();
    let wallet_account_balance = rent.minimum_balance(wallet_account_len);

    let wallet_address = Pubkey::new_unique();
    let account = Keypair::new();
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account(
                &payer.pubkey(),
                &account.pubkey(),
                wallet_account_balance,
                wallet_account_len as u64,
                &spl_wallet_2025::id(),
            ),
            initialize_account(
                &spl_wallet_2025::id(),
                &account.pubkey(),
                &wallet_mint_address,
                &wallet_address,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[&payer, &account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let mut instruction = create_associated_wallet_account_idempotent(
        &payer.pubkey(),
        &wallet_address,
        &wallet_mint_address,
        &spl_wallet_2025::id(),
    );
    instruction.accounts[1] = AccountMeta::new(account.pubkey(), false); // <-- Invalid associated_account_address

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    assert_eq!(
        banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(0, InstructionError::InvalidSeeds)
    );
}
