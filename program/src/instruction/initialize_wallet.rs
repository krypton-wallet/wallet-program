use std::collections::HashSet;

use crate::{prelude::*, state::UserProfile};

use super::InitializeWalletArgs;

pub fn process_initialize_wallet(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitializeWalletArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let system_program = next_account_info(&mut account_info_iter);

    // ensure authority_info is signer
    if !authority_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure profile_info is writable
    if !profile_info.is_writable {
        return Err(KryptonError::NotWriteable.into());
    }

    // ensure profile_info PDA corresponds to authority_info
    let (profile_pda, profile_bump_seed) = get_profile_pda(authority_info.key, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // ensure recovery threshold is valid
    if args.recovery_threshold > MAX_GUARDIANS {
        return Err(KryptonError::TooManyGuardians.into());
    }

    msg!("account checks complete");

    // create profile account inside profile pda iff pda account does not exist
    if **profile_info.try_borrow_lamports()? == 0 {
        msg!("no lamports, creating new PDA account....");
        let create_profile_account_instruction = create_account(
            authority_info.key,
            &profile_pda,
            Rent::get()?.minimum_balance(DATA_LEN),
            DATA_LEN as u64,
            program_id,
        );

        // invoke CPI to create profile account
        invoke_signed(
            &create_profile_account_instruction,
            &[
                profile_info.clone(),
                authority_info.clone(),
                system_program.expect("system program").clone(),
            ],
            &[&[PDA_SEED, authority_info.key.as_ref(), &[profile_bump_seed]]],
        )?;
    } else if profile_info.data_is_empty() {
        msg!("no space in PDA account, allocating space....");

        let assign_instruction = assign(&profile_pda, program_id);
        // Invoke CPI to assign my program to own PDA
        invoke_signed(
            &assign_instruction,
            &[
                profile_info.clone(),
                authority_info.clone(),
                system_program.expect("system program").clone(),
            ],
            &[&[PDA_SEED, authority_info.key.as_ref(), &[profile_bump_seed]]],
        )?;

        profile_info.realloc(DATA_LEN, false)?;
    }

    // create ProfileHeader
    let initial_data = UserProfile {
        seed: *authority_info.key,
        authority: *authority_info.key,
        recovery_threshold: args.recovery_threshold,
        guardians: vec![Guardian::default(); MAX_GUARDIANS as usize]
            .try_into()
            .unwrap(),
        recovery: Pubkey::default(),
        recovered: HashSet::new(),
    };
    let initial_data_len = initial_data.try_to_vec()?.len();
    msg!("data len: {}, expected: {}", initial_data_len, DATA_LEN);

    initial_data.serialize(&mut &mut profile_info.try_borrow_mut_data()?[..initial_data_len])?;

    Ok(())
}
