use borsh::BorshDeserialize;
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
        add_recovery_guardians, add_recovery_sign, initialize_recovery, initialize_wallet,
        modify_recovery_threshold, recover_token, recover_wallet, remove_recovery_guardians,
        transfer_native_sol, transfer_token, wrap_instruction, KryptonInstruction,
    },
    state::{get_profile_pda, verify_recovery_state, ProfileHeader, PDA_SEED},
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
                modify_recovery_threshold::process_modify_recovery_threshold(
                    program_id, accounts, args,
                )
            }
            KryptonInstruction::InitializeRecovery => {
                msg!("Instruction: InitializeRecovery");
                initialize_recovery::process_initialize_recovery(program_id, accounts)
            }
            KryptonInstruction::AddRecoverySign => {
                msg!("Instruction: AddRecoverySign");
                add_recovery_sign::process_add_recovery_sign(program_id, accounts)
            }
            KryptonInstruction::RecoverWallet => {
                msg!("Instruction: RecoverWallet");
                recover_wallet::process_recover_wallet(program_id, accounts)
            }
            KryptonInstruction::RecoverToken => {
                msg!("Instruction: RecoverToken");
                recover_token::process_recover_token(program_id, accounts)
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
