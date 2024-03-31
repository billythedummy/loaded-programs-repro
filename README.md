# loaded-programs-repro

Minimum reproduction of change in behaviour of `LoadedPrograms` in `ProgramTest` after solana version 1.17.6

## Setup

The `1.17.6/` folder and `1.18.9/` folder contains the exact same code of loading the spl stake pool program into a `ProgramTest` by calling [`ProgramTest::add_account()`](https://docs.rs/solana-program-test/latest/solana_program_test/struct.ProgramTest.html#method.add_account) on `spl-stake-pool-prog.json` and `spl-stake-pool-prog-data.json` (the program and program data accounts cloned from mainnet), and then trying to execute a transaction that calls the program against the started program test.

In 1.17.6, the program executes normally.

In 1.18.9, it hangs and eventually fails with `RpcError::DeadlineExceeded`. If simulating instead, it throws [TransactionError::InvalidProgramForExecution](https://docs.rs/solana-sdk/latest/solana_sdk/transaction/enum.TransactionError.html#variant.InvalidProgramForExecution). This happens even after warping the context past the program's last upgrade slot and epoch.

## Run

`cd` into the respective folders and run `cargo test`
