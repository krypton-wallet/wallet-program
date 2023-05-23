use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::assign,
    system_program::check_id,
};

use crate::{
    error::KryptonError,
    state::{get_profile_pda, DATA_LEN, PDA_SEED},
};

use super::WrapInstructionArgs;

pub fn process_wrap_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: WrapInstructionArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();
    msg!("Instruction: WrapInstruction");

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let custom_program = next_account_info(&mut account_info_iter)?;

    // ensure the specified amount of accounts are passed in
    if (args.num_accounts + 3) < accounts.len() as u8 {
        return Err(KryptonError::NotEnoughAccounts.into());
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
    let (profile_pda, bump_seed) = get_profile_pda(authority_info.key, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    msg!("account checks complete");

    // make a copy of PDA data
    let mut old_data: [u8; DATA_LEN] = [0; DATA_LEN];
    old_data.copy_from_slice(&profile_info.data.borrow()[..]);

    // check if custom_program is system_program
    let mut system_program: Option<&AccountInfo> = None;
    if check_id(custom_program.key) {
        system_program = Some(custom_program);
    }

    let mut custom_infos = Vec::with_capacity(args.num_accounts as usize);
    let mut custom_metas = Vec::with_capacity(args.num_accounts as usize);
    for _ in 0..args.num_accounts {
        // populate custom_infos
        let custom_account_info = next_account_info(&mut account_info_iter)?;
        custom_infos.push(custom_account_info.clone());

        // check if any passed in account is system_program
        if check_id(custom_account_info.key) {
            system_program = Some(custom_account_info);
        }

        // populate custom_metas
        let new_meta = if profile_pda == *custom_account_info.key {
            AccountMeta::new(*custom_account_info.key, true)
        } else if custom_account_info.is_writable {
            AccountMeta::new(*custom_account_info.key, custom_account_info.is_signer)
        } else {
            AccountMeta::new_readonly(*custom_account_info.key, custom_account_info.is_signer)
        };
        custom_metas.push(new_meta);
    }

    // system_program is present so assign PDA to be system-owned
    if system_program.is_some() {
        profile_info.realloc(0, false)?;
        profile_info.assign(&solana_program::system_program::ID);
    }

    // call CPI instruction
    msg!("data: {:?}", args.custom_data);
    let instr = Instruction::new_with_bytes(
        *custom_program.key,
        args.custom_data.as_slice(),
        custom_metas,
    );
    invoke_signed(
        &instr,
        custom_infos.as_slice(),
        &[&[PDA_SEED, authority_info.key.as_ref(), &[bump_seed]]],
    )?;
    msg!("invoked_signed!");

    // system_program is present so reassign PDA to KryptonProgram
    if system_program.is_some() {
        let assign_ix = assign(&profile_pda, program_id);
        invoke_signed(
            &assign_ix,
            &[profile_info.clone(), system_program.unwrap().clone()],
            &[&[PDA_SEED, authority_info.key.as_ref(), &[bump_seed]]],
        )?;
        profile_info.realloc(DATA_LEN, false)?;
        profile_info.data.borrow_mut()[..].copy_from_slice(&old_data);
    }

    Ok(())
}
