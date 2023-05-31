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
    for _ in 2..accounts.len() {
        let guardian_info = next_account_info(&mut account_info_iter)?;

        // delete guardian if present; return Err otherwise
        if profile_data.guardians.remove(guardian_info.key).is_none() {
            return Err(KryptonError::GuardianNotFound.into());
        }
        msg!("deleted guardian {:?}", guardian_info.key);
    }
    msg!("new guardian list: {:?}", profile_data.guardians);

    profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

    Ok(())
}
