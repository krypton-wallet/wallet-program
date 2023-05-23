use crate::{
    error::KryptonError,
    state::{get_profile_pda, Guardian, ProfileHeader, MAX_GUARDIANS},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::RemoveRecoveryGuardianArgs;

pub fn process_remove_recovery_guardians(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: RemoveRecoveryGuardianArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.into_iter();
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

    let (profile_pda, _) = get_profile_pda(authority_info.key, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    msg!("account checks complete");

    let mut profile_data = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

    msg!("old guardian list: {:?}", profile_data.guardians);

    // delete guardian(s)
    let mut guardians: Vec<Guardian> = profile_data.guardians.into_iter().collect();
    for _ in 0..args.num_guardians {
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
