use solana_program::program_memory::sol_memcpy;

use crate::prelude::*;
use crate::state::{Guard, GuardAccount, NativeSolTransferGuard, NativeSolTransferInterval};

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
    let system_program = next_account_info(&mut account_info_iter);

    // ensure authority_info is signer
    if !authority_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure profile_info and guard_info are writable
    if !profile_info.is_writable || !guard_info.is_writable {
        return Err(KryptonError::NotWriteable.into());
    }

    let profile_data =
        ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?[..PROFILE_HEADER_LEN])?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    // ensure seed_info is valid
    let (profile_pda, _) = get_profile_pda(&profile_data.seed, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // ensure guard_info is valid
    let guard_seeds = &[b"guard", profile_info.key.as_ref()];
    let guard_bump = assert_derivation(
        &crate::id(),
        guard_info,
        guard_seeds,
        KryptonError::InvalidAccountAddress,
    )?;
    let guard_signer_seeds = &[b"guard", profile_info.key.as_ref(), &[guard_bump]];

    msg!("account checks complete");

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
    let required_lamports = Rent::get()?.minimum_balance(serialized.len());

    let create_guard_ix = create_account(
        authority_info.key,
        guard_info.key,
        required_lamports,
        serialized.len() as u64,
        &crate::id(),
    );
    // invoke CPI to create guard account
    invoke_signed(
        &create_guard_ix,
        &[
            authority_info.clone(),
            guard_info.clone(),
            system_program.expect("system program").clone(),
        ],
        &[guard_signer_seeds],
    )?;

    msg!("guard account created");

    // ensure there is enough SOL to transfer
    let profile_data_lamports = Rent::get()?.minimum_balance(profile_info.data_len());
    if **profile_info.try_borrow_lamports()? < (profile_data_lamports + required_lamports) {
        return Err(ProgramError::InsufficientFunds);
    }

    // debit profile_info and credit dest
    let mut profile_lamports = profile_info.try_borrow_mut_lamports()?;
    let mut authority_lamports = authority_info.try_borrow_mut_lamports()?;

    **profile_lamports = profile_lamports
        .checked_sub(required_lamports)
        .ok_or(KryptonError::ArithmeticOverflow)?;
    **authority_lamports = authority_lamports
        .checked_add(required_lamports)
        .ok_or(KryptonError::ArithmeticOverflow)?;

    // write bytes to new account
    sol_memcpy(
        *guard_info.try_borrow_mut_data()?,
        serialized.as_slice(),
        serialized.len(),
    );

    Ok(())
}
