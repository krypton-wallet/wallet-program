use crate::{
    error::KryptonError,
    state::{get_profile_pda, PDA_SEED},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{self, state::Account as TokenAccount};

use super::TransferTokenArgs;

pub fn process_transfer_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferTokenArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let token_account_info = next_account_info(&mut account_info_iter)?;
    let dest_token_account_info = next_account_info(&mut account_info_iter)?;
    let token_program = next_account_info(&mut account_info_iter)?;

    // ensure authority_info is signer
    if !authority_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure token_account_info and dest_token_account_info are writable
    if !token_account_info.is_writable || !dest_token_account_info.is_writable {
        return Err(KryptonError::NotWriteable.into());
    }

    // ensure profile_info PDA corresponds to authority_info
    let (profile_pda, bump_seed) = get_profile_pda(authority_info.key, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    msg!("account checks complete");

    // unpack token account to get amount
    let token_account = TokenAccount::unpack(&token_account_info.try_borrow_data()?)?;
    msg!("amount: {}, total: {}", args.amount, token_account.amount);

    // ensure ATA has enough tokens to transfer
    if token_account.amount < args.amount {
        return Err(ProgramError::InsufficientFunds);
    }

    msg!("transfering mint...");
    let transfer_ix = spl_token::instruction::transfer(
        token_program.key,
        token_account_info.key,
        dest_token_account_info.key,
        &profile_pda,
        &[&profile_pda],
        args.amount,
    )?;

    invoke_signed(
        &transfer_ix,
        &[
            token_program.clone(),
            token_account_info.clone(),
            dest_token_account_info.clone(),
            authority_info.clone(),
            profile_info.clone(),
        ],
        &[&[PDA_SEED, authority_info.key.as_ref(), &[bump_seed]]],
    )?;
    msg!("finished transfer of mint");

    Ok(())
}
