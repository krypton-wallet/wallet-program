use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};
use spl_token::instruction::{close_account, transfer};

use crate::instruction::RecoveryInstruction;
use crate::{error::RecoveryError, state::ProfileHeader};

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = RecoveryInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let account_info_iter = &mut accounts.iter();

        match instruction {
            RecoveryInstruction::InitializeSocialWallet {
                acct_len,
                recovery_threshold,
            } => {
                msg!("Instruction: InitializeSocialWallet");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let system_program_info = next_account_info(account_info_iter)?;

                // Store list of guardians (social recovery list)
                let mut guardians = Vec::with_capacity(acct_len.into());
                for _ in 0..acct_len {
                    let guardian_account_info = next_account_info(account_info_iter)?;
                    guardians.push(*guardian_account_info.key);
                }

                /*
                    allocate space for 10 recovery accounts (guardian) in profile account data
                    1: recovery_threshold
                    32: executor
                    4: size of vector of guardians
                    32 * 10: space for 10 guardians
                */

                let data_len = (1 + 32 + 4 + 32 * 10) as u64;
                msg!("Number of bytes of data: {}", data_len);

                // find pda of profile account for given authority
                let (profile_pda, profile_bump_seed) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );

                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // create profile account inside profile pda
                let create_profile_account_instruction = create_account(
                    authority_info.key,
                    &profile_pda,
                    Rent::get()?.minimum_balance(data_len as usize),
                    data_len.into(),
                    program_id,
                );

                // Invoke CPI to create profile account
                invoke_signed(
                    &create_profile_account_instruction,
                    &[
                        profile_info.clone(),
                        authority_info.clone(),
                        system_program_info.clone(),
                    ],
                    &[&[
                        b"profile",
                        authority_info.key.as_ref(),
                        &[profile_bump_seed],
                    ]],
                )?;

                // Create ProfileHeader and Serialize using borsh
                let initial_data = ProfileHeader {
                    recovery_threshold,
                    executor: Pubkey::default(),
                    guardians,
                };
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                initial_data
                    .serialize(&mut &mut profile_info.try_borrow_mut_data()?[..initial_data_len])?;

                Ok(())
            }
            RecoveryInstruction::AddToRecoveryList { acct_len } => {
                msg!("Instruction: AddToRecovery");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of profile account for given authority
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );

                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // Add the guardian data into profile program data
                let profile_data = &mut profile_info.try_borrow_mut_data()?;
                let old_acct_len = profile_data[33];
                let old_data_len = (old_acct_len * 32 + 37) as usize;

                // assert that total number of guardians are less than or equal to 10
                if old_acct_len + acct_len > 10 {
                    return Err(RecoveryError::TooManyGuardians.into());
                }

                // Deserialize into ProfileHeader from profile program data
                let mut initial_data =
                    ProfileHeader::try_from_slice(&profile_data[..old_data_len])?;

                // Log existing guardians
                msg!("Old Guardian List: ");
                for i in 0..old_acct_len {
                    msg!(
                        "{}: {:x?}",
                        i,
                        initial_data.guardians[i as usize].to_bytes()
                    );
                }

                // Add new guardian into deserialized struct
                for i in 0..acct_len {
                    let guardian_account_info = next_account_info(account_info_iter)?;
                    msg!(
                        "newly added guardian {}: {:x?}",
                        i,
                        guardian_account_info.key.to_bytes()
                    );
                    initial_data.guardians.push(*guardian_account_info.key);
                }

                // Log new guardians after add
                msg!("New Guardian List: ");
                for i in 0..old_acct_len + acct_len {
                    msg!(
                        "{}: {:x?}",
                        i,
                        initial_data.guardians[i as usize].to_bytes()
                    );
                }

                // Serialize struct (after adding guardians) into profile program data
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                let mut writer = &mut profile_data[..initial_data_len];
                initial_data.serialize(&mut writer)?;

                Ok(())
            }
            RecoveryInstruction::ModifyRecoveryList { acct_len } => {
                msg!("Instruction: ModifyRecoveryList");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of profile account for given authority
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );

                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // Add the guardian data into profile program data
                let profile_data = &mut profile_info.try_borrow_mut_data()?;
                let old_acct_len = profile_data[33];
                let old_data_len = (old_acct_len * 32 + 37) as usize;

                // Deserialize into ProfileHeader from profile program data
                let mut initial_data =
                    ProfileHeader::try_from_slice(&profile_data[..old_data_len])?;

                // Log existing guardians
                msg!("Old Guardian List: ");
                for i in 0..old_acct_len {
                    msg!(
                        "{}: {:x?}",
                        i,
                        initial_data.guardians[i as usize].to_bytes()
                    );
                }

                // Add new guardian into deserialized struct
                for _ in 0..acct_len {
                    let old_guardian_info = next_account_info(account_info_iter)?;
                    let new_guardian_info = next_account_info(account_info_iter)?;
                    let old_guardian_pk = old_guardian_info.key;
                    let new_guardian_pk = new_guardian_info.key;

                    // check if the key to be modified is in the data
                    if !initial_data.guardians.contains(old_guardian_pk) {
                        return Err(RecoveryError::ModifiedGuardianNotFound.into());
                    }

                    // get index of old guardian key in the data
                    let index = initial_data
                        .guardians
                        .iter()
                        .position(|&k| k == *old_guardian_pk)
                        .unwrap();

                    // replace old with new key in the index of old key
                    initial_data.guardians[index] = *new_guardian_pk;
                    msg!(
                        "replace old {:x?} with new {:x?}",
                        old_guardian_pk.to_bytes(),
                        new_guardian_pk.to_bytes()
                    );
                }

                // print all guardians
                msg!("New Guardian List: ");
                for i in 0..initial_data.guardians.len() {
                    msg!("{}: {:x?}", i, initial_data.guardians[i].to_bytes());
                }

                // Serialize struct (after adding guardians) into profile program data
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                let mut writer = &mut profile_data[..initial_data_len];
                initial_data.serialize(&mut writer)?;

                Ok(())
            }
            RecoveryInstruction::DeleteFromRecoveryList { acct_len } => {
                msg!("Instruction: DeleteFromRecoveryList");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of profile account for given authority
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );

                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // Add the guardian data into profile program data
                let profile_data = &mut profile_info.try_borrow_mut_data()?;
                let old_acct_len = profile_data[33];
                let recovery_threshold = profile_data[0];
                let old_data_len = (old_acct_len * 32 + 37) as usize;

                msg!("old acct len: {}", old_acct_len);
                msg!("acct_len: {}", acct_len);
                msg!("recover thres: {}", recovery_threshold);

                // assert that total number of guardians are greater than or equal to the recovery threshold
                if old_acct_len - acct_len < recovery_threshold {
                    return Err(RecoveryError::NotEnoughGuardians.into());
                }

                // Deserialize into ProfileHeader from profile program data
                let mut initial_data =
                    ProfileHeader::try_from_slice(&profile_data[..old_data_len])?;

                // print all old guardians
                msg!("Old Guardian List: ");
                for i in 0..initial_data.guardians.len() {
                    msg!("{}: {:x?}", i, initial_data.guardians[i].to_bytes());
                }

                // Delete guardian from deserialized struct
                for _ in 0..acct_len {
                    let guardian_info = next_account_info(account_info_iter)?;
                    let guardian_pk = guardian_info.key;

                    // check if the key to be deleted is in the data
                    if !initial_data.guardians.contains(guardian_pk) {
                        return Err(RecoveryError::DeletedGuardianNotFound.into());
                    }

                    // get index of guardian key to be deleted in the data
                    let index = initial_data
                        .guardians
                        .iter()
                        .position(|&k| k == *guardian_pk)
                        .unwrap();

                    // replace old with new key in the index of old key
                    initial_data.guardians.remove(index);
                    msg!("deleted guardian {:x?}", guardian_pk.to_bytes());
                }

                // print all guardians
                msg!("New Guardian List: ");
                for i in 0..initial_data.guardians.len() {
                    msg!("{}: {:x?}", i, initial_data.guardians[i].to_bytes());
                }

                // Serialize struct (after adding guardians) into profile program data
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                let mut writer = &mut profile_data[..initial_data_len];
                initial_data.serialize(&mut writer)?;

                // Zero out the deleted space
                for i in initial_data_len..old_data_len {
                    profile_data[i] = 0;
                }

                Ok(())
            }
            RecoveryInstruction::ModifyRecoveryThreshold { new_threshold } => {
                msg!("Instruction: ModifyRecoveryThreshold");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of profile account for given authority
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );

                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                if new_threshold > 10 || new_threshold <= 0 {
                    return Err(RecoveryError::InvalidRecoveryThreshold.into());
                }

                // Add the guardian data into profile program data
                let profile_data = &mut profile_info.try_borrow_mut_data()?;
                profile_data[0] = new_threshold;

                Ok(())
            }
            RecoveryInstruction::RecoverWallet { acct_len } => {
                msg!("Instruction: RecoverWallet");

                let profile_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;
                let system_program_info = next_account_info(account_info_iter)?;
                let executor_info = next_account_info(account_info_iter)?;

                // check if authorities are signers
                if !authority_info.is_signer || !new_authority_info.is_signer {
                    return Err(ProgramError::InvalidArgument);
                }

                // find pda of profile account for given authority
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }
                msg!("Old Profile PDA: {}", profile_pda);

                // Add the guardian data into profile program data
                let profile_data = &mut profile_info.try_borrow_mut_data()?;
                let recovery_threshold = profile_data[0];
                let old_acct_len = profile_data[33];
                let old_data_len = (old_acct_len * 32 + 37) as usize;
                profile_data[1..33].clone_from_slice(&executor_info.key.to_bytes());

                if recovery_threshold > acct_len {
                    return Err(RecoveryError::NotEnoughGuardiansToRecover.into());
                }

                // Deserialize into ProfileHeader from profile program data
                let initial_data = ProfileHeader::try_from_slice(&profile_data[..old_data_len])?;
                let mut guardian_infos = Vec::with_capacity(acct_len.into());

                for _ in 0..acct_len {
                    let guardian_info = next_account_info(account_info_iter)?;
                    let guardian_pk = guardian_info.key;

                    // check if the input guardian key is authorized (in profile program data)
                    if !initial_data.guardians.contains(guardian_pk) {
                        return Err(RecoveryError::NotAuthorizedToRecover.into());
                    }

                    // check if guardian passed in is a signer
                    if !guardian_info.is_signer {
                        return Err(RecoveryError::NotAuthorizedToRecover.into());
                    }

                    guardian_infos.push(guardian_info);
                }

                // // Create ProfileHeader and Serialize into profile pda (updated with executor)
                // let profile_data_with_executor = ProfileHeader {
                //     recovery_threshold,
                //     executor: *executor_info.key,
                //     guardians: initial_data.guardians.clone(),
                // };
                // let profile_data_with_executor_len = profile_data_with_executor.try_to_vec()?.len();
                // msg!("data len: {}", profile_data_with_executor_len);
                // msg!("Serializing with executor data...");
                // profile_data_with_executor.serialize(
                //     &mut &mut profile_info.try_borrow_mut_data()?[..profile_data_with_executor_len],
                // )?;

                let guardians = initial_data.guardians.clone();

                // allocate space for 10 recovery accounts (guardian) in new profile account data
                let data_len = (37 + 32 * 10) as u64;

                // find pda of new profile account for new authority
                let (new_profile_pda, new_bump_seed) = Pubkey::find_program_address(
                    &[b"profile", new_authority_info.key.as_ref()],
                    program_id,
                );
                msg!("New Profile PDA: {}", new_profile_pda);

                /*
                // create a new profile account
                let create_profile_account_instruction = create_account(
                    new_authority_info.key,
                    &new_profile_pda,
                    Rent::get()?.minimum_balance(data_len as usize),
                    data_len.into(),
                    program_id,
                );

                let mut account_infos = vec![
                    new_profile_info.clone(),
                    new_authority_info.clone(),
                    system_program_info.clone(),
                ];

                for i in 0..acct_len {
                    account_infos.push(guardian_infos[i as usize].clone());
                }

                // Invoke CPI to create profile account
                invoke_signed(
                    &create_profile_account_instruction,
                    &account_infos,
                    &[&[
                        b"profile",
                        new_authority_info.key.as_ref(),
                        &[new_bump_seed],
                    ]],
                )?;
                */

                // Create ProfileHeader and Serialize using borsh
                let initial_data = ProfileHeader {
                    recovery_threshold,
                    executor: Pubkey::default(),
                    guardians,
                };
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                initial_data.serialize(
                    &mut &mut new_profile_info.try_borrow_mut_data()?[..initial_data_len],
                )?;

                Ok(())
            }

            RecoveryInstruction::TransferToNewTokenAccount {
                amount,
                recovery_mode,
            } => {
                msg!("Instruction: TransferToNewTokenAccount");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let old_token_account_info = next_account_info(account_info_iter)?;
                let new_token_account_info = next_account_info(account_info_iter)?;
                let token_program_info = next_account_info(account_info_iter)?;

                if !authority_info.is_signer {
                    return Err(ProgramError::InvalidArgument);
                }

                let mut executor_key = Pubkey::default();
                if recovery_mode == 1 {
                    let executor_info = next_account_info(account_info_iter)?;
                    if !executor_info.is_signer {
                        return Err(RecoveryError::ExecutorNotSigner.into());
                    }
                    executor_key = *executor_info.key;
                }

                // find pda of profile account for given authority
                let (profile_pda, bump_seed) = Pubkey::find_program_address(
                    &[b"profile", authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }
                msg!("amount: {}", amount);

                msg!("transfering mint...");
                let transfer_ix = transfer(
                    token_program_info.key,
                    old_token_account_info.key,
                    new_token_account_info.key,
                    &profile_pda,
                    &[&profile_pda],
                    amount,
                )?;

                invoke_signed(
                    &transfer_ix,
                    &[
                        token_program_info.clone(),
                        old_token_account_info.clone(),
                        new_token_account_info.clone(),
                        authority_info.clone(),
                        profile_info.clone(),
                    ],
                    &[&[b"profile", authority_info.key.as_ref(), &[bump_seed]]],
                )?;
                msg!("finished transfer of mint");

                let profile_cloned = profile_info.clone();
                // Get profile data and prepare for deserialization
                let profile_data = &profile_info.try_borrow_data()?;
                let old_acct_len = profile_data[33];
                let old_data_len = (old_acct_len * 32 + 37) as usize;
                msg!("profile data gotten");

                // Deserialize into ProfileHeader from profile program data
                let initial_data = ProfileHeader::try_from_slice(&profile_data[..old_data_len])?;
                msg!("deserialized");

                // close sender token account if executor from pda matches authority
                // (meaning executor invoked the transfer and we should close the account)
                if recovery_mode == 1 && initial_data.executor == executor_key {
                    msg!("closing account...");
                    let close_ix = close_account(
                        token_program_info.key,
                        old_token_account_info.key,
                        new_token_account_info.key,
                        &profile_pda,
                        &[&profile_pda],
                    )?;

                    invoke_signed(
                        &close_ix,
                        &[
                            token_program_info.clone(),
                            old_token_account_info.clone(),
                            new_token_account_info.clone(),
                            authority_info.clone(),
                            profile_cloned,
                        ],
                        &[&[b"profile", authority_info.key.as_ref(), &[bump_seed]]],
                    )?;
                    msg!("closed");
                }

                Ok(())
            }
        }
    }
}
