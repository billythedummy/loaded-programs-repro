#[tokio::test]
async fn test() {
    use sanctum_solana_test_utils::{workspace_root_dir, ExtendedProgramTest};
    use solana_program_test::ProgramTest;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signer::Signer,
        transaction::Transaction,
    };

    mod spl_stake_pool_prog {
        solana_program::declare_id!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");
    }

    let repo_root = workspace_root_dir().join("../");
    let pt = ProgramTest::default()
        .add_account_from_file(repo_root.join("spl-stake-pool-prog.json"))
        .add_account_from_file(repo_root.join("spl-stake-pool-prog-data.json"));

    let (mut bc, payer, rbh) = pt.start().await;

    let random_stake_pool_ix_that_will_fail = Instruction {
        program_id: spl_stake_pool_prog::ID,
        accounts: vec![AccountMeta {
            pubkey: payer.pubkey(),
            is_signer: true,
            is_writable: false,
        }],
        data: vec![],
    };

    // Should see program execution logs and
    // err should be
    // TransactionError(InstructionError(0, BorshIoError("Unknown")))
    bc.process_transaction(Transaction::new_signed_with_payer(
        &[random_stake_pool_ix_that_will_fail],
        Some(&payer.pubkey()),
        &[&payer],
        rbh,
    ))
    .await
    .unwrap();
}
