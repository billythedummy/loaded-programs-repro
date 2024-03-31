#[tokio::test]
async fn test() {
    use sanctum_solana_test_utils::{workspace_root_dir, ExtendedProgramTest};
    use solana_program_test::{ProgramTest, ProgramTestContext};
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signer::Signer,
        transaction::Transaction,
    };

    mod spl_stake_pool_prog {
        solana_program::declare_id!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");
    }

    const SPL_STAKE_POOL_PROG_LAST_UPDATE_SLOT_PLUS_ONE: u64 = 238_419_617;
    const SPL_STAKE_POOL_PROG_LAST_UPDATE_EPOCH: u64 = 551;

    let repo_root = workspace_root_dir().join("../");
    let pt = ProgramTest::default()
        .add_account_from_file(repo_root.join("spl-stake-pool-prog.json"))
        .add_account_from_file(repo_root.join("spl-stake-pool-prog-data.json"));

    let mut ctx = pt.start_with_context().await;
    // try to warp forward past program's last update slot
    ctx.warp_to_epoch(SPL_STAKE_POOL_PROG_LAST_UPDATE_EPOCH)
        .unwrap();
    ctx.warp_to_slot(SPL_STAKE_POOL_PROG_LAST_UPDATE_SLOT_PLUS_ONE)
        .unwrap();
    ctx.warp_forward_force_reward_interval_end().unwrap(); // one more just for good measure
    let ProgramTestContext {
        banks_client: mut bc,
        payer,
        last_blockhash: rbh,
        ..
    } = ctx;

    let random_stake_pool_ix_that_will_fail = Instruction {
        program_id: spl_stake_pool_prog::ID,
        accounts: vec![AccountMeta {
            pubkey: payer.pubkey(),
            is_signer: true,
            is_writable: false,
        }],
        data: vec![],
    };

    let tx = Transaction::new_signed_with_payer(
        &[random_stake_pool_ix_that_will_fail],
        Some(&payer.pubkey()),
        &[&payer],
        rbh,
    );

    // Should hang and eventually fail with
    // RpcError(DeadlineExceeded)
    // No program logs are shown because the program is never executed
    bc.process_transaction(tx).await.unwrap();

    // comment out process_transaction above and uncomment this block.
    // --nocapture simulating should show more details
    /*
    let sim = bc.simulate_transaction(tx).await.unwrap();
    eprintln!("{:?}", sim.result.unwrap()); // should be Err(InvalidProgramForExecution)
    eprintln!("{:?}", sim.simulation_details.unwrap()); // should be everything empty
     */
}
