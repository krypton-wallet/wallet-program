use solana_program::program_memory::sol_memcpy;
use solana_program::system_instruction;

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
    let _system_program_info = next_account_info(&mut account_info_iter)?;

    let profile = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

    let guard_seeds = &[b"guard", profile_info.key.as_ref()];
    let guard_bump = assert_derivation(
        &crate::id(),
        guard_info,
        guard_seeds,
        KryptonError::InvalidAccountAddress,
    )?;
    let guard_signer_seeds = &[b"guard", profile_info.key.as_ref(), &[guard_bump]];

    let profile_seeds = &[PDA_SEED, profile.authority.as_ref()];
    let profile_bump = assert_derivation(
        &crate::id(),
        profile_info,
        profile_seeds,
        KryptonError::InvalidAccountAddress,
    )?;

    let _profile_signer_seeds = &[PDA_SEED, profile.authority.as_ref(), &[profile_bump]];

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

    // create_or_allocate_account_raw(
    //     crate::id(),
    //     guard_info,
    //     system_program_info,
    //     profile_info,
    //     serialized.len(),
    //     profile_signer_seeds,
    //     guard_signer_seeds,
    // )?;
    let create_profile_account_instruction = create_account(
        authority_info.key,
        guard_info.key,
        required_lamports,
        serialized.len() as u64,
        &crate::id(),
    );
    // invoke CPI to create profile account
    invoke_signed(
        &create_profile_account_instruction,
        &[authority_info.clone(), guard_info.clone()],
        &[guard_signer_seeds],
    )?;
    msg!("created account with system program");

    // ensure there is enough SOL to transfer
    if **profile_info.try_borrow_lamports()? < required_lamports {
        return Err(ProgramError::InsufficientFunds);
    }

    // debit profile_info and credit dest
    **profile_info.try_borrow_mut_lamports()? -= required_lamports;
    **authority_info.try_borrow_mut_lamports()? += required_lamports;


    // write bytes to new account
    sol_memcpy(
        *guard_info.try_borrow_mut_data()?,
        serialized.as_slice(),
        serialized.len(),
    );

    Ok(())
}

pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
    new_account_signer_seeds: &[&[u8]],
) -> ProgramResult {
    let rent = &Rent::get()?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        // // ensure there is enough SOL to transfer
        // if **payer_info.try_borrow_lamports()? < required_lamports {
        //     return Err(ProgramError::InsufficientFunds);
        // }

        // // debit profile_info and credit dest
        // **payer_info.try_borrow_mut_lamports()? -= required_lamports;
        // **new_account_info.try_borrow_mut_lamports()? += required_lamports;

        invoke_signed(
            &system_instruction::transfer(payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
            &[signer_seeds],
        )?;
    }

    let accounts = &[new_account_info.clone(), system_program_info.clone()];

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        accounts,
        &[new_account_signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        accounts,
        &[new_account_signer_seeds],
    )?;

    Ok(())
}
