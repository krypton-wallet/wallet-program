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

use super::ModifyRecoveryThresholdArgs;


pub fn process_modify_recovery_threshold(program_id: &Pubkey, accounts: &[AccountInfo], args: ModifyRecoveryThresholdArgs) -> ProgramResult {
    let mut account_info_iter = accounts.into_iter();
                msg!("Instruction: ModifyRecoveryThreshold");

                let profile_info = next_account_info(&mut account_info_iter)?;
                let authority_info = next_account_info(&mut account_info_iter)?;

                // ensure the new threshold is valid
                if args.new_threshold > MAX_GUARDIANS || args.new_threshold == 0 {
                    return Err(KryptonError::InvalidRecoveryThreshold.into());
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

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                // update the recovery threshold
                profile_data.recovery_threshold = args.new_threshold;
                profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

                Ok(())

}