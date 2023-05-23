use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::KryptonError, state::get_profile_pda};

use super::TransferNativeSOLArgs;

pub fn process_transfer_native_sol(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferNativeSOLArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let dest = next_account_info(&mut account_info_iter)?;

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

    // ensure there is enough SOL to transfer
    if **profile_info.try_borrow_lamports()? < args.amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // debit profile_info and credit dest
    **profile_info.try_borrow_mut_lamports()? -= args.amount;
    **dest.try_borrow_mut_lamports()? += args.amount;

    msg!("amount: {}", args.amount);

    Ok(())
}
