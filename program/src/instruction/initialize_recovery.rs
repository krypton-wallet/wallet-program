use crate::prelude::*;

pub fn process_initialize_recovery(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let new_profile_info = next_account_info(&mut account_info_iter)?;
    let new_authority_info = next_account_info(&mut account_info_iter)?;

    // ensure new_authority_info and guardian_info are signer
    if !new_authority_info.is_signer {
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

    let new_profile_data = UserProfile::try_from_slice(&new_profile_info.try_borrow_data()?)?;

    // ensure new_authority_info is valid
    if new_profile_data.authority != *new_authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    let mut profile_data = UserProfile::try_from_slice(&profile_info.try_borrow_data()?)?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    // if new recovery, then update recovery and unset all guardian signatures
    if *new_profile_info.key != profile_data.recovery {
        msg!("new recovery: {:?}", new_authority_info.key);
        profile_data.recovery = *new_profile_info.key;
        for (_, has_signed) in profile_data.guardians.iter_mut() {
            *has_signed = false;
        }
    }

    profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

    Ok(())
}
