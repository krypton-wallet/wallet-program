use crate::{prelude::*, state::verify_recovery_state};

pub fn process_recover_token(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut account_info_iter = accounts.into_iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let new_profile_info = next_account_info(&mut account_info_iter)?;
    let new_authority_info = next_account_info(&mut account_info_iter)?;
    let old_token_account_info = next_account_info(&mut account_info_iter)?;
    let new_token_account_info = next_account_info(&mut account_info_iter)?;
    let token_program = next_account_info(&mut account_info_iter)?;

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
    let old_token_account = TokenAccount::unpack(&old_token_account_info.try_borrow_data()?)?;
    let amount = old_token_account.amount;
    msg!("amount: {}", amount);

    // transfer tokens to new_token_account_info
    msg!("transfering mint...");
    let transfer_ix = spl_token::instruction::transfer(
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
    let close_ix = spl_token::instruction::close_account(
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
