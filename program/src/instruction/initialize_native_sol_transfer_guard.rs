use solana_program::clock::Clock;

use crate::prelude::*;
use crate::state::{Guard, NativeSolTransferGuard, NativeSolTransferInterval, GuardAccount};

use super::InitializeNativeSolTransferGuardArgs;

pub fn process_initialize_native_sol_transfer_guard(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitializeNativeSolTransferGuardArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let guard_info = next_account_info(&mut account_info_iter)?;

    // assert the derivation path is correct,
    // create and assign the new account
    // create guard
    // serialize guard and write bytes to account

    let sol_guard = NativeSolTransferGuard::new(
        profile_info.key,
        args.transfer_amount,
        NativeSolTransferInterval::Day,
    );
    let guard_account = GuardAccount {
        guard: Guard::NativeSolTransfer(sol_guard),
        target: profile_info.key.to_owned()
    };

    let serialized = guard_account.try_to_vec()?;

    Ok(())
}
