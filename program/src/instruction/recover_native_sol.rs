use crate::{prelude::*, state::verify_recovery_state};

pub fn process_recover_native_sol(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let new_profile_info = next_account_info(&mut account_info_iter)?;
    let new_authority_info = next_account_info(&mut account_info_iter)?;

    // ensure new_authority_info is signer
    if !new_authority_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure profile_info and new_profile_info are writable
    if !profile_info.is_writable || !new_profile_info.is_writable {
        return Err(KryptonError::NotWriteable.into());
    }

    // ensure profile_info PDA corresponds to authority_info
    let (profile_pda, _) = get_profile_pda(authority_info.key, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // ensure new_profile_info PDA corresponds to new_authority_info
    let (new_profile_pda, _) = get_profile_pda(new_authority_info.key, program_id);
    if new_profile_pda != *new_profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    msg!("checks complete");

    let profile_data = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

    // ensure recovery is happening for new_profile_info
    if profile_data.recovery != *new_profile_info.key {
        return Err(KryptonError::NotAuthorizedToRecover.into());
    }

    // ensure new_profile_info can recover
    if !verify_recovery_state(&profile_data) {
        return Err(KryptonError::MissingGuardianSignatures.into());
    }

    msg!("recovery checks complete");

    // transfer all the lamports from profile_info to new_profile_info
    let balance = profile_info.lamports();
    **new_profile_info.try_borrow_mut_lamports()? = balance
        .checked_add(new_profile_info.lamports())
        .ok_or(KryptonError::Overflow)?;
    **profile_info.try_borrow_mut_lamports()? = 0;
    profile_info.data.borrow_mut().fill(0);

    msg!("amount: {}", balance);

    Ok(())
}
