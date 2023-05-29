use crate::{
    error::KryptonError,
    prelude::ProfileHeader,
    state::{get_profile_pda, PDA_SEED},
};
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

    let profile_data = ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?[..64])?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    // ensure seed_info is valid
    let (profile_pda, bump_seed) = get_profile_pda(&profile_data.seed, program_id);
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
            profile_info.clone(),
        ],
        &[&[PDA_SEED, profile_data.seed.as_ref(), &[bump_seed]]],
    )?;
    msg!("finished transfer of mint");

    Ok(())
}
