use crate::{
    prelude::*,
    state::{verify_recovery_state, UserProfile, PROFILE_HEADER_LEN},
};

pub fn process_recover_wallet(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
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
    msg!("old Profile PDA: {}", profile_pda);
    msg!("new Profile PDA: {}", new_profile_pda);

    let profile_data = UserProfile::try_from_slice(&profile_info.try_borrow_data()?)?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    // ensure recovery is happening for new_profile_info
    if profile_data.recovery != *new_profile_info.key {
        return Err(KryptonError::NotAuthorizedToRecover.into());
    }

    // ensure new_profile_info can recover
    if !verify_recovery_state(&profile_data) {
        return Err(KryptonError::MissingGuardianSignatures.into());
    }

    msg!("recovery checks complete");

    let mut new_profile_data = UserProfile::try_from_slice(&new_profile_info.try_borrow_data()?)?;

    // ensure new_authority_info is valid
    if new_profile_data.authority != *new_authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    // insert old profile_info into recovered set
    new_profile_data.recovered.insert(profile_pda);

    // ensure required number of recovered accounts are passed in
    if profile_data.recovered.len() + 4 < accounts.len() {
        return Err(KryptonError::MissingRecoveredAccounts.into());
    }

    // loop through recovered accounts
    for _ in 4..accounts.len() {
        let recovered_info = next_account_info(&mut account_info_iter)?;

        if profile_data.recovered.contains(recovered_info.key) {
            // add to new recovered set
            new_profile_data.recovered.insert(*recovered_info.key);

            // update authority
            let mut recovered_data =
                ProfileHeader::try_from_slice(&recovered_info.try_borrow_data()?)?;
            recovered_data.authority = new_profile_data.authority;
            recovered_data.serialize(&mut &mut recovered_info.try_borrow_mut_data()?[..])?;
        }
    }

    // update new_profile_info
    new_profile_data.serialize(&mut &mut new_profile_info.try_borrow_mut_data()?[..])?;

    // convert profile_info to ProfileHeader with new authority
    let profile_header = ProfileHeader {
        seed: profile_data.seed,
        authority: new_profile_data.authority,
    };
    profile_info.realloc(PROFILE_HEADER_LEN, false)?;
    profile_header.serialize(&mut &mut profile_info.try_borrow_mut_data()?[..])?;

    Ok(())
}
