use crate::prelude::*;

use super::AddRecoveryGuardianArgs;

pub fn process_add_recovery_guardians(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddRecoveryGuardianArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;

    // ensure the specified amount of guardians are passed in
    if (args.num_guardians + 2) < accounts.len() as u8 {
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

    let mut profile_data = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

    // assert that total number of guardians are less than or equal to MAX_GUARDIANS
    let guardian_count = profile_data
        .guardians
        .into_iter()
        .filter(|guardian| guardian.pubkey != Pubkey::default())
        .count();
    if (guardian_count as u8 + args.num_guardians) > MAX_GUARDIANS {
        return Err(KryptonError::TooManyGuardians.into());
    }

    msg!("old guardian count: {}", guardian_count);
    msg!("old guardian list: {:?}", profile_data.guardians);

    // add new guardian(s)
    for i in 0..args.num_guardians {
        let guardian_account_info = next_account_info(&mut account_info_iter)?;
        msg!(
            "newly added guardian {}: {:?}",
            i,
            guardian_account_info.key
        );
        let new_guardian = Guardian {
            pubkey: *guardian_account_info.key,
            has_signed: false,
        };
        profile_data.guardians[guardian_count + i as usize] = new_guardian;
    }

    msg!(
        "new guardian count: {}",
        guardian_count + args.num_guardians as usize
    );
    msg!("new guardian list: {:?}", profile_data.guardians);

    profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

    Ok(())
}
