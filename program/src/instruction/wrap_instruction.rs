use super::WrapInstructionArgs;
use crate::prelude::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_memory::sol_memcpy,
    system_program::{check_id, ID},
};

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

    let profile_data =
        ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?[..PROFILE_HEADER_LEN])?;

    // ensure seed_info is valid
    let (profile_pda, bump_seed) = get_profile_pda(&profile_data.seed, program_id);
    if profile_pda != *profile_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    msg!("account checks complete");

    // make a copy of PDA data
    let old_data = profile_info.try_borrow_data()?.try_to_vec()?;

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
        profile_info.assign(&ID);
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
        &[&[PDA_SEED, profile_data.seed.as_ref(), &[bump_seed]]],
    )?;
    msg!("invoked_signed!");

    // system_program is present so reassign PDA to KryptonProgram
    if system_program.is_some() {
        let assign_ix = assign(&profile_pda, program_id);
        invoke_signed(
            &assign_ix,
            &[profile_info.clone(), system_program.unwrap().clone()],
            &[&[PDA_SEED, profile_data.seed.as_ref(), &[bump_seed]]],
        )?;
        profile_info.realloc(old_data.len(), false)?;
        sol_memcpy(
            &mut profile_info.data.borrow_mut()[..],
            &old_data.as_slice(),
            old_data.len(),
        );
    }

    Ok(())
}
