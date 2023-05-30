use super::TransferNativeSOLArgs;
use crate::prelude::*;

pub fn process_transfer_native_sol(
    _program_id: &Pubkey,
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

    let profile_data =
        ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?[..PROFILE_HEADER_LEN])?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
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
