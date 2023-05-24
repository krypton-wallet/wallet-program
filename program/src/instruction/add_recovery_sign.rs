use crate::prelude::*;

pub fn process_add_recovery_sign(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let new_profile_info = next_account_info(&mut account_info_iter)?;
    let new_authority_info = next_account_info(&mut account_info_iter)?;
    let guardian_info = next_account_info(&mut account_info_iter)?;

    // ensure guardian_info is signer
    if !guardian_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure profile_info is writable
    if !profile_info.is_writable {
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

    msg!("account checks complete");

    let mut profile_data = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

    // ensure recovery is happening for new_profile_info
    if profile_data.recovery != *new_profile_info.key {
        return Err(KryptonError::NotAuthorizedToRecover.into());
    }

    // get index of signing guardian key
    let idx = profile_data
        .guardians
        .into_iter()
        .position(|guardian| guardian.pubkey == *guardian_info.key);

    // ensure guardian is present
    if idx.is_none() {
        return Err(KryptonError::GuardianNotFound.into());
    }

    // add guardian signature
    profile_data.guardians[idx.unwrap()].has_signed = true;
    profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

    Ok(())
}
