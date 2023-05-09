use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{assign, create_account},
    system_program::check_id,
    sysvar::Sysvar,
};
use spl_token::{
    instruction::{close_account, transfer},
    state::Account as TokenAccount,
};

use crate::{
    error::KryptonError,
    state::{verify_recovery_state, ProfileHeader},
};
use crate::{
    instruction::KryptonInstruction,
    state::{Guardian, DATA_LEN, MAX_GUARDIANS, PDA_SEED},
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

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let system_program = next_account_info(account_info_iter)?;

                // ensure authority_info is signer
                if !authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure profile_info is writable
                if !profile_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, profile_bump_seed) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // ensure recovery threshold is valid
                if args.recovery_threshold > MAX_GUARDIANS {
                    return Err(KryptonError::TooManyGuardians.into());
                }

                msg!("account checks complete");

                // create profile account inside profile pda iff pda account does not exist
                if **profile_info.try_borrow_lamports()? == 0 {
                    msg!("no lamports, creating new PDA account....");
                    let create_profile_account_instruction = create_account(
                        authority_info.key,
                        &profile_pda,
                        Rent::get()?.minimum_balance(DATA_LEN),
                        DATA_LEN as u64,
                        program_id,
                    );

                    // invoke CPI to create profile account
                    invoke_signed(
                        &create_profile_account_instruction,
                        &[
                            profile_info.clone(),
                            authority_info.clone(),
                            system_program.clone(),
                        ],
                        &[&[PDA_SEED, authority_info.key.as_ref(), &[profile_bump_seed]]],
                    )?;
                } else if profile_info.data_is_empty() {
                    msg!("no space in PDA account, allocating space....");

                    let assign_instruction = assign(&profile_pda, program_id);
                    // Invoke CPI to assign my program to own PDA
                    invoke_signed(
                        &assign_instruction,
                        &[
                            profile_info.clone(),
                            authority_info.clone(),
                            system_program.clone(),
                        ],
                        &[&[PDA_SEED, authority_info.key.as_ref(), &[profile_bump_seed]]],
                    )?;

                    profile_info.realloc(DATA_LEN as usize, false)?;
                }

                // create ProfileHeader
                let initial_data = ProfileHeader {
                    authority: *authority_info.key,
                    recovery_threshold: args.recovery_threshold,
                    guardians: vec![Guardian::default(); MAX_GUARDIANS as usize]
                        .try_into()
                        .unwrap(),
                    priv_scan: args.priv_scan,
                    priv_spend: args.priv_spend,
                    recovery: Pubkey::default(),
                };
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}, expected: {}", initial_data_len, DATA_LEN);

                initial_data
                    .serialize(&mut &mut profile_info.try_borrow_mut_data()?[..initial_data_len])?;

                Ok(())
            }
            KryptonInstruction::TransferToken(args) => {
                msg!("Instruction: TransferToken");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let token_account_info = next_account_info(account_info_iter)?;
                let dest_token_account_info = next_account_info(account_info_iter)?;
                let token_program = next_account_info(account_info_iter)?;

                // ensure authority_info is signer
                if !authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure token_account_info and dest_token_account_info are writable
                if !token_account_info.is_writable || !dest_token_account_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, bump_seed) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                // unpack token account to get amount
                let token_account = TokenAccount::unpack(&token_account_info.try_borrow_data()?)?;
                msg!("amount: {}, total: {}", args.amount, token_account.amount);

                // ensure ATA has enough tokens to transfer
                if token_account.amount < args.amount {
                    return Err(ProgramError::InsufficientFunds.into());
                }

                msg!("transfering mint...");
                let transfer_ix = transfer(
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
            KryptonInstruction::TransferNativeSOL(args) => {
                msg!("Instruction: TransferNativeSOL");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let dest = next_account_info(account_info_iter)?;

                // ensure authority_info is signer
                if !authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure profile_info is writable
                if !profile_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                // ensure there is enough SOL to transfer
                if **profile_info.try_borrow_lamports()? < args.amount {
                    return Err(ProgramError::InsufficientFunds.into());
                }

                // debit profile_info and credit dest
                **profile_info.try_borrow_mut_lamports()? -= args.amount;
                **dest.try_borrow_mut_lamports()? += args.amount;

                msg!("amount: {}", args.amount);

                Ok(())
            }
            KryptonInstruction::WrapInstruction(args) => {
                msg!("Instruction: WrapInstruction");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let custom_program = next_account_info(account_info_iter)?;

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
                let (profile_pda, bump_seed) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
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
                    let custom_account_info = next_account_info(account_info_iter)?;
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
                        AccountMeta::new_readonly(
                            *custom_account_info.key,
                            custom_account_info.is_signer,
                        )
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
                    let assign_ix = assign(&profile_pda, &program_id);
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
            KryptonInstruction::UpdateSecret(args) => {
                msg!("Instruction: UpdateSecret");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // ensure authority_info is signer
                if !authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure profile_info is writable
                if !profile_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                // update priv_scan and priv_spend
                profile_data.priv_scan = args.priv_scan;
                profile_data.priv_spend = args.priv_spend;
                profile_data.serialize(&mut &mut profile_info.try_borrow_mut_data()?[..])?;

                Ok(())
            }
            KryptonInstruction::AddRecoveryGuardians(args) => {
                msg!("Instruction: AddRecoveryGuardians");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // ensure the specified amount of guardians are passed in
                if (args.num_guardians + 2) < accounts.len() as u8 {
                    return Err(KryptonError::NotEnoughGuardians.into());
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
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                // assert that total number of guardians are less than or equal to MAX_GUARDIANS
                let guardian_count = profile_data
                    .guardians
                    .into_iter()
                    .filter(|guardian| guardian.pubkey != Pubkey::default())
                    .count();
                if (guardian_count as u8 + args.num_guardians) > MAX_GUARDIANS {
                    return Err(KryptonError::TooManyGuardians.into());
                }

                msg!("old guardian count: {}", guardian_count);
                msg!("old guardian list: {:?}", profile_data.guardians);

                // find shard idxs to assign to new guardians
                let used_shard_idxs = profile_data
                    .guardians
                    .into_iter()
                    .filter(|guardian| guardian.pubkey != Pubkey::default())
                    .map(|guardian| guardian.shard_idx);
                let mut shard_idxs = Vec::with_capacity(args.num_guardians as usize);
                for i in 0..MAX_GUARDIANS {
                    if !used_shard_idxs.clone().any(|idx| idx == i) {
                        shard_idxs.push(i);
                        if shard_idxs.len() == args.num_guardians as usize {
                            break;
                        }
                    }
                }

                // add new guardian(s)
                for i in 0..args.num_guardians {
                    let guardian_account_info = next_account_info(account_info_iter)?;
                    msg!(
                        "newly added guardian {}: {:x?}",
                        i,
                        guardian_account_info.key.to_bytes()
                    );
                    let new_guardian = Guardian {
                        pubkey: *guardian_account_info.key,
                        shard_idx: shard_idxs[i as usize],
                        has_signed: false,
                    };
                    profile_data.guardians[guardian_count + i as usize] = new_guardian;
                }

                msg!(
                    "new guardian count: {}",
                    guardian_count + args.num_guardians as usize
                );
                msg!("new guardian list: {:?}", profile_data.guardians);

                profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

                Ok(())
            }
            KryptonInstruction::RemoveRecoveryGuardians(args) => {
                msg!("Instruction: DeleteRecoveryGuardians");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // ensure the specified amount of guardians are passed in
                if (args.num_guardians + 2) < accounts.len() as u8 {
                    return Err(KryptonError::NotEnoughGuardians.into());
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
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                msg!("old guardian list: {:?}", profile_data.guardians);

                // delete guardian(s)
                let mut guardians: Vec<Guardian> = profile_data.guardians.into_iter().collect();
                for _ in 0..args.num_guardians {
                    let guardian_info = next_account_info(account_info_iter)?;

                    // get index of guardian key to be deleted
                    let idx = guardians
                        .iter()
                        .position(|guardian| guardian.pubkey == *guardian_info.key);

                    // ensure guardian is present
                    if !idx.is_none() {
                        return Err(KryptonError::GuardianNotFound.into());
                    }

                    guardians.remove(idx.unwrap());
                    msg!("deleted guardian {:x?}", guardian_info.key.to_bytes());
                }

                msg!("new guardian list: {:?}", profile_data.guardians);

                while guardians.len() < MAX_GUARDIANS as usize {
                    guardians.push(Guardian::default());
                }

                profile_data.guardians = guardians.try_into().unwrap();
                profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

                Ok(())
            }
            KryptonInstruction::ModifyRecoveryThreshold(args) => {
                msg!("Instruction: ModifyRecoveryThreshold");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // ensure the new threshold is valid
                if args.new_threshold > MAX_GUARDIANS || args.new_threshold == 0 {
                    return Err(KryptonError::InvalidRecoveryThreshold.into());
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
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                // update the recovery threshold
                profile_data.recovery_threshold = args.new_threshold;
                profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

                Ok(())
            }
            KryptonInstruction::AddRecoverySign(_args) => {
                msg!("Instruction: AddRecoverySign");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;
                let guardian_info = next_account_info(account_info_iter)?;

                // ensure new_authority_info and guardian_info are signer
                if !new_authority_info.is_signer || !guardian_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure profile_info is writable
                if !profile_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // ensure new_profile_info PDA corresponds to new_authority_info
                let (new_profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, new_authority_info.key.as_ref()],
                    program_id,
                );
                if new_profile_pda != *new_profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("account checks complete");

                let mut profile_data =
                    ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?)?;

                // if new recovery, then update recovery and unset other guardian signatures
                if *new_profile_info.key != profile_data.recovery {
                    msg!("new recovery: {:?}", new_authority_info.key.to_bytes());
                    profile_data.recovery = *new_profile_info.key;
                    for guardian in profile_data.guardians.iter_mut() {
                        guardian.has_signed = false;
                    }
                }

                // get index of signing guardian key
                let idx = profile_data
                    .guardians
                    .into_iter()
                    .position(|guardian| guardian.pubkey == *guardian_info.key);

                // ensure guardian is present
                if !idx.is_none() {
                    return Err(KryptonError::GuardianNotFound.into());
                }

                // add guardian signature
                profile_data.guardians[idx.unwrap()].has_signed = true;
                profile_data.serialize(&mut &mut profile_info.data.borrow_mut()[..])?;

                Ok(())
            }
            KryptonInstruction::RecoverWallet(_args) => {
                msg!("Instruction: RecoverWallet");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;

                // ensure new_authority_info is signer
                if !new_authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure new_profile_info is writable
                if !new_profile_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // ensure new_profile_info PDA corresponds to new_authority_info
                let (new_profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, new_authority_info.key.as_ref()],
                    program_id,
                );
                if new_profile_pda != *new_profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                msg!("checks complete");
                msg!("old Profile PDA: {}", profile_pda);
                msg!("new Profile PDA: {}", new_profile_pda);

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

                // copy over data to new_profile_info
                profile_data.serialize(&mut &mut new_profile_info.try_borrow_mut_data()?[..])?;

                Ok(())
            }
            KryptonInstruction::RecoverToken(_args) => {
                msg!("Instruction: RecoverToken");

                let profile_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let new_profile_info = next_account_info(account_info_iter)?;
                let new_authority_info = next_account_info(account_info_iter)?;
                let old_token_account_info = next_account_info(account_info_iter)?;
                let new_token_account_info = next_account_info(account_info_iter)?;
                let token_program = next_account_info(account_info_iter)?;

                // ensure new_authority_info is signer
                if !new_authority_info.is_signer {
                    return Err(KryptonError::NotSigner.into());
                }

                // ensure old_token_account_info and new_token_account_info are writable
                if !old_token_account_info.is_writable || !new_token_account_info.is_writable {
                    return Err(KryptonError::NotWriteable.into());
                }

                // ensure profile_info PDA corresponds to authority_info
                let (profile_pda, bump_seed) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // ensure new_profile_info PDA corresponds to new_authority_info
                let (new_profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, new_authority_info.key.as_ref()],
                    program_id,
                );
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
                let old_token_account =
                    TokenAccount::unpack(&old_token_account_info.try_borrow_data()?)?;
                let amount = old_token_account.amount;
                msg!("amount: {}", amount);

                // transfer tokens to new_token_account_info
                msg!("transfering mint...");
                let transfer_ix = transfer(
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
                let close_ix = close_account(
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
            KryptonInstruction::RecoverNativeSOL(_args) => {
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
                let (profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, authority_info.key.as_ref()],
                    program_id,
                );
                if profile_pda != *profile_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                // ensure new_profile_info PDA corresponds to new_authority_info
                let (new_profile_pda, _) = Pubkey::find_program_address(
                    &[PDA_SEED, new_authority_info.key.as_ref()],
                    program_id,
                );
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
