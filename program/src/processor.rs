use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{close_account, transfer},
    state::Account as TokenAccount,
};

use crate::{
    error::KryptonError,
    instruction::{
        add_recovery_guardians, initialize_wallet, remove_recovery_guardians, transfer_native_sol,
        transfer_token, wrap_instruction, KryptonInstruction, modify_recovery_threshold,
    },
    state::{get_profile_pda, verify_recovery_state, ProfileHeader, MAX_GUARDIANS, PDA_SEED},
};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = KryptonInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let account_info_iter = &mut accounts.iter();

        match instruction {
            KryptonInstruction::InitializeWallet(args) => {
                msg!("Instruction: InitializeWallet");
                initialize_wallet::process_initialize_wallet(program_id, accounts, args)
            }
            KryptonInstruction::TransferToken(args) => {
                msg!("Instruction: TransferToken");
                transfer_token::process_transfer_token(program_id, accounts, args)
            }
            KryptonInstruction::TransferNativeSOL(args) => {
                msg!("Instruction: TransferNativeSol");
                transfer_native_sol::process_transfer_native_sol(program_id, accounts, args)
            }
            KryptonInstruction::WrapInstruction(args) => {
                msg!("Instruction: WrapInstruction");
                wrap_instruction::process_wrap_instruction(program_id, accounts, args)
            }
            KryptonInstruction::AddRecoveryGuardians(args) => {
                msg!("Instruction: AddRecoveryGuardians");
                add_recovery_guardians::process_add_recovery_guardians(program_id, accounts, args)
            }
            KryptonInstruction::RemoveRecoveryGuardians(args) => {
                msg!("Instruction: RemoveRecoveryGuardians");
                remove_recovery_guardians::process_remove_recovery_guardians(
                    program_id, accounts, args,
                )
            }
            KryptonInstruction::ModifyRecoveryThreshold(args) => {
                msg!("Instruction: ModifyRecoveryThreshold");
                modify_recovery_threshold::process_modify_recovery_threshold(program_id, accounts, args)
            }
            KryptonInstruction::InitializeRecovery => {
                msg!("Instruction: InitializeRecovery");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;

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

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                // if new recovery, then update recovery and unset other guardian signatures
                if *new_profile_info.key != profile_data.recovery {
                    msg!("new recovery: {:?}", new_authority_info.key);
                    profile_data.recovery = *new_profile_info.key;
                    for guardian in profile_data.guardians.iter_mut() {
                        guardian.has_signed = false;
                    }
                }

                profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

                Ok(())
            }
            KryptonInstruction::AddRecoverySign => {
                msg!("Instruction: AddRecoverySign");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;
                let guardian_info = next_account_info(account_info_iter)?;

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

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

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
            KryptonInstruction::RecoverWallet => {
                msg!("Instruction: RecoverWallet");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;

                // ensure new_authority_info is signer
                if !new_authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure new_profile_info is writable
                if !new_profile_info.is_writable {
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

                // copy over data to new_profile_info
                profile_data.serialize(&mut &mut new_profile_info.try_borrow_mut_data()?[..])?;

                Ok(())
            }
            KryptonInstruction::RecoverToken => {
                msg!("Instruction: RecoverToken");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;
                let old_token_account_info = next_account_info(account_info_iter)?;
                let new_token_account_info = next_account_info(account_info_iter)?;
                let token_program = next_account_info(account_info_iter)?;

                // ensure new_authority_info is signer
                if !new_authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure old_token_account_info and new_token_account_info are writable
                if !old_token_account_info.is_writable || !new_token_account_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, bump_seed) = get_profile_pda(authority_info.key, program_id);
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // ensure new_profile_info PDA corresponds to new_authority_info
                let (new_profile_pda, _) = get_profile_pda(new_authority_info.key, program_id);
                if new_profile_pda != *new_profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

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

                // unpack old token account to get amount
                let old_token_account =
                    TokenAccount::unpack(&old_token_account_info.try_borrow_data()?)?;
                let amount = old_token_account.amount;
                msg!("amount: {}", amount);

                // transfer tokens to new_token_account_info
                msg!("transfering mint...");
                let transfer_ix = transfer(
                    token_program.key,
                    old_token_account_info.key,
                    new_token_account_info.key,
                    &profile_pda,
                    &[&profile_pda],
                    amount,
                )?;
                invoke_signed(
                    &transfer_ix,
                    &[
                        token_program.clone(),
                        old_token_account_info.clone(),
                        new_token_account_info.clone(),
                        authority_info.clone(),
                        profile_info.clone(),
                    ],
                    &[&[PDA_SEED, authority_info.key.as_ref(), &[bump_seed]]],
                )?;
                msg!("finished transfer of mint");

                // close old_token_account
                msg!("closing token account...");
                let close_ix = close_account(
                    token_program.key,
                    old_token_account_info.key,
                    new_token_account_info.key,
                    &profile_pda,
                    &[&profile_pda],
                )?;
                invoke_signed(
                    &close_ix,
                    &[
                        token_program.clone(),
                        old_token_account_info.clone(),
                        new_token_account_info.clone(),
                        authority_info.clone(),
                        profile_info.clone(),
                    ],
                    &[&[PDA_SEED, authority_info.key.as_ref(), &[bump_seed]]],
                )?;
                msg!("token account closed");

                Ok(())
            }
            KryptonInstruction::RecoverNativeSOL => {
                msg!("Instruction: RecoverNativeSOL");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;

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
        }
    }
}
