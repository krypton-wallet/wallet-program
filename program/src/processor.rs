use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::{
    add_recovery_guardians, add_recovery_sign, initialize_recovery, initialize_wallet,
    modify_recovery_threshold, recover_native_sol, recover_token, recover_wallet,
    remove_recovery_guardians, transfer_native_sol, transfer_token, wrap_instruction,
    KryptonInstruction,
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
            KryptonInstruction::AddRecoveryGuardians => {
                msg!("Instruction: AddRecoveryGuardians");
                add_recovery_guardians::process_add_recovery_guardians(program_id, accounts)
            }
            KryptonInstruction::RemoveRecoveryGuardians => {
                msg!("Instruction: RemoveRecoveryGuardians");
                remove_recovery_guardians::process_remove_recovery_guardians(program_id, accounts)
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
                recover_native_sol::process_recover_native_sol(program_id, accounts)
            }
        }
    }
}
