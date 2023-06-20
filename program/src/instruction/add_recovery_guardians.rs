use mpl_utils::resize_or_reallocate_account_raw;
use solana_program::program_memory::sol_memcpy;

use crate::prelude::*;

pub fn process_add_recovery_guardians(
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

    // ensure profile_info PDA corresponds to authority_info
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

    // assert that total number of guardians are less than or equal to MAX_GUARDIANS
    let guardian_count = profile_data.guardians.len();
    if (guardian_count + accounts.len()) > MAX_GUARDIANS as usize {
        return Err(KryptonError::TooManyGuardians.into());
    }

    msg!("old guardian count: {}", guardian_count);
    msg!("old guardian list: {:?}", profile_data.guardians);

    let system_program = next_account_info(&mut account_info_iter)?;
    msg!("system program id: {:?}", system_program.key);
    // add new guardian(s)
    for i in 3..accounts.len() {
        let guardian_account_info = next_account_info(&mut account_info_iter)?;
        msg!(
            "newly added guardian {}: {:?}",
            i,
            guardian_account_info.key
        );
        profile_data
            .guardians
            .insert(*guardian_account_info.key, false);
    }

    msg!("new guardian count: {}", profile_data.guardians.len());
    msg!("new guardian list: {:?}", profile_data.guardians);

    let serialized_profile_data = profile_data.try_to_vec()?;
    resize_or_reallocate_account_raw(profile_info, authority_info, system_program, serialized_profile_data.len())?;
    // profile_data.serialize(&mut buf)?;
    sol_memcpy(*profile_info.try_borrow_mut_data()?, &serialized_profile_data, serialized_profile_data.len());
    // TODO: resize account

    Ok(())
}
