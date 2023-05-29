use solana_program::program_memory::sol_memcpy;

use crate::prelude::*;
use crate::state::{Guard, GuardAccount, NativeSolTransferGuard, NativeSolTransferInterval};

use super::InitializeNativeSolTransferGuardArgs;

pub fn process_initialize_native_sol_transfer_guard(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitializeNativeSolTransferGuardArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let guard_info = next_account_info(&mut account_info_iter)?;
    let system_program_info = next_account_info(&mut account_info_iter)?;

    let profile = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

    let guard_seeds = &[b"guard", profile_info.key.as_ref()];
    assert_derivation(
        &crate::id(),
        guard_info,
        guard_seeds,
        KryptonError::InvalidAccountAddress,
    )?;

    let profile_seeds = &[PDA_SEED, profile.authority.as_ref()];
    let profile_bump = assert_derivation(
        &crate::id(),
        profile_info,
        profile_seeds,
        KryptonError::InvalidAccountAddress,
    )?;

    let profile_signer_seeds = &[PDA_SEED, profile.authority.as_ref(), &[profile_bump]];

    let sol_guard = NativeSolTransferGuard::new(
        profile_info.key,
        args.transfer_amount,
        NativeSolTransferInterval::Day,
    );
    let guard_account = GuardAccount {
        guard: Guard::NativeSolTransfer(sol_guard),
        target: profile_info.key.to_owned(),
    };

    let serialized = guard_account.try_to_vec()?;

    create_or_allocate_account_raw(
        crate::id(),
        guard_info,
        system_program_info,
        profile_info,
        serialized.len(),
        profile_signer_seeds,
    )?;

    // write bytes to new account
    sol_memcpy(
        *guard_info.try_borrow_mut_data()?,
        serialized.as_slice(),
        serialized.len(),
    );

    Ok(())
}
