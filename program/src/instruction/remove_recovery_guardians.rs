use crate::prelude::*;

pub fn process_remove_recovery_guardians(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;

    // ensure guardians are passed in
    if accounts.len() < 3 {
        return Err(KryptonError::NotEnoughGuardians.into());
    }

    // ensure authority_info is signer
    if !authority_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure profile_info is writable
    if !profile_info.is_writable {
        return Err(KryptonError::NotWriteable.into());
    }

    let (profile_pda, _) = get_profile_pda(authority_info.key, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    msg!("account checks complete");

    let mut profile_data = UserProfile::try_from_slice(&profile_info.try_borrow_data()?)?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    msg!("old guardian list: {:?}", profile_data.guardians);

    // delete guardian(s)
    let mut guardians: Vec<Guardian> = profile_data.guardians.into_iter().collect();
    for _ in 2..accounts.len() {
        let guardian_info = next_account_info(&mut account_info_iter)?;

        // get index of guardian key to be deleted
        let idx = guardians
            .iter()
            .position(|guardian| guardian.pubkey == *guardian_info.key);

        // ensure guardian is present
        if idx.is_none() {
            return Err(KryptonError::GuardianNotFound.into());
        }

        guardians.remove(idx.unwrap());
        msg!("deleted guardian {:?}", guardian_info.key);
    }

    msg!("new guardian list: {:?}", profile_data.guardians);

    while guardians.len() < MAX_GUARDIANS as usize {
        guardians.push(Guardian::default());
    }

    profile_data.guardians = guardians.try_into().unwrap();
    profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

    Ok(())
}
