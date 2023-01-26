use std::{str::FromStr};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, 
    program::{invoke_signed},
    program_error::ProgramError,
    system_instruction::create_account,
    pubkey::Pubkey,
    rent::Rent, sysvar::Sysvar, 
};

use crate::{error::EchoError, state::WalletHeader};
use crate::instruction::EchoInstruction;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let account_info_iter = &mut accounts.iter();
        let usdc_pk_str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        let usdc_pk = Pubkey::from_str(usdc_pk_str).unwrap();

        match instruction {
            EchoInstruction::InitializeSocialWallet {
                acct_len,
                recovery_threshold,
            } => {
                msg!("Instruction: InitializeSocialWallet");
                
                let wallet_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;
                let system_program_info = next_account_info(account_info_iter)?;

                // Store list of guardians (social recovery list)
                let mut guardians = Vec::with_capacity(acct_len.into());
                for _ in 0..acct_len {
                    let guardian_account_info = next_account_info(account_info_iter)?;
                    guardians.push(*guardian_account_info.key);
                }

                // find pda of wallet program that corresponds to usdc bucket for given authority
                let (wallet_pda, bump_seed) = Pubkey::find_program_address(
                    &[
                        b"bucket",
                        authority_info.key.as_ref(),
                        // usdc mint public key
                        usdc_pk.as_ref(),
                    ],
                    program_id,
                );

                if wallet_pda != *wallet_info.key {
                    return Err(ProgramError::InvalidSeeds)
                }

                // allocate space for 10 recovery accounts (guardian) in program data
                let data_len = (5 + 32 * 10) as u64;
                msg!("Number of bytes of data: {}", data_len);

                // create bucket account
                let create_account_instruction = create_account(
                    authority_info.key, 
                    &wallet_pda, 
                    Rent::get()?.minimum_balance( data_len as usize), 
                    data_len.into(),
                    program_id
                );

                // Invoke CPI to create USDC bucket in wallet program and sign it with seed of wallet
                invoke_signed(
                    &create_account_instruction, 
                    &[
                        wallet_info.clone(),
                        authority_info.clone(),
                        system_program_info.clone(),
                    ],
                    &[
                        &[
                            b"bucket", 
                            authority_info.key.as_ref(), 
                            usdc_pk.as_ref(),
                            &[bump_seed]
                        ]
                    ],
                )?;

                // Create WalletHeader and Serialize using borsh
                let initial_data = WalletHeader{
                    recovery_threshold,
                    guardians,
                };
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                initial_data.serialize(&mut &mut wallet_info.try_borrow_mut_data()?[..initial_data_len])?;

                Ok(())
            }
            EchoInstruction::AddToRecoveryList { acct_len } => {
                msg!("Instruction: AddToRecovery");

                let wallet_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of wallet program that corresponds to usdc bucket for given authority
                let (wallet_pda, _) = Pubkey::find_program_address(
                    &[
                        b"bucket",
                        authority_info.key.as_ref(),
                        // usdc mint public key
                        usdc_pk.as_ref(),
                    ],
                    program_id,
                );

                if wallet_pda != *wallet_info.key {
                    return Err(ProgramError::InvalidSeeds)
                }

                // Add the guardian data into wallet program data
                let wallet_data = &mut wallet_info.try_borrow_mut_data()?;
                let old_acct_len = wallet_data[1];
                let old_data_len = (old_acct_len * 32 + 5) as usize;

                // assert that total number of guardians are less than or equal to 10
                assert!(old_acct_len + acct_len <= 10);

                // Deserialize into WalletHeader from wallet program data
                let mut initial_data = WalletHeader::try_from_slice(&wallet_data[..old_data_len])?;

                // Add new guardian into deserialized struct
                for i in 0..acct_len {
                    let guardian_account_info = next_account_info(account_info_iter)?;
                    msg!("newly added guardian {}: {:?}", i, guardian_account_info.key.to_bytes());
                    initial_data.guardians.push(*guardian_account_info.key);
                }

                // Serialize struct (after adding guardians) into wallet program data
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                let mut writer = &mut wallet_data[..initial_data_len];
                initial_data.serialize(&mut writer)?;

                Ok(())
            }
            EchoInstruction::ModifyRecoveryList {
                acct_len,
            } => {
                msg!("Instruction: ModifyRecoveryList");

                let wallet_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of wallet program that corresponds to usdc bucket for given authority
                let (wallet_pda, _) = Pubkey::find_program_address(
                    &[
                        b"bucket",
                        authority_info.key.as_ref(),
                        // usdc mint public key
                        usdc_pk.as_ref(),
                    ],
                    program_id,
                );

                if wallet_pda != *wallet_info.key {
                    return Err(ProgramError::InvalidSeeds)
                }

                // Add the guardian data into wallet program data
                let wallet_data = &mut wallet_info.try_borrow_mut_data()?;
                let old_acct_len = wallet_data[1];
                let old_data_len = (old_acct_len * 32 + 5) as usize;

                // assert that total number of guardians are less than or equal to 10
                assert!(old_acct_len + acct_len <= 10, "too many guardians added");

                // Deserialize into WalletHeader from wallet program data
                let mut initial_data = WalletHeader::try_from_slice(&wallet_data[..old_data_len])?;

                // Add new guardian into deserialized struct
                for _ in 0..acct_len {
                    let old_guardian_info = next_account_info(account_info_iter)?;
                    let new_guardian_info = next_account_info(account_info_iter)?;
                    let old_guardian_pk = old_guardian_info.key;
                    let new_guardian_pk = new_guardian_info.key;

                    // check if the key to be modified is in the data
                    assert!(initial_data.guardians.contains(old_guardian_pk), "the guardian to be replaced is not in the data");

                    // get index of old guardian key in the data
                    let index = initial_data.guardians.iter().position(|&k| k == *old_guardian_pk).unwrap();

                    // replace old with new key in the index of old key
                    initial_data.guardians[index] = *new_guardian_pk;
                    msg!("replace old {:?} with new {:?}", old_guardian_pk.to_bytes(), new_guardian_pk.to_bytes());
                }

                // print all guardians
                for i in 0..initial_data.guardians.len() {
                    msg!("Guardian {}: {:?}", i, initial_data.guardians[i].to_bytes());
                }

                // Serialize struct (after adding guardians) into wallet program data
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                let mut writer = &mut wallet_data[..initial_data_len];
                initial_data.serialize(&mut writer)?;

                Ok(())
            }
            EchoInstruction::DeleteFromRecoveryList {
                acct_len,
            } => {
                msg!("Instruction: DeleteFromRecoveryList");

                let wallet_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of wallet program that corresponds to usdc bucket for given authority
                let (wallet_pda, _) = Pubkey::find_program_address(
                    &[
                        b"bucket",
                        authority_info.key.as_ref(),
                        // usdc mint public key
                        usdc_pk.as_ref(),
                    ],
                    program_id,
                );

                if wallet_pda != *wallet_info.key {
                    return Err(ProgramError::InvalidSeeds)
                }

                // Add the guardian data into wallet program data
                let wallet_data = &mut wallet_info.try_borrow_mut_data()?;
                let old_acct_len = wallet_data[1];
                let recovery_threshold = wallet_data[0];
                let old_data_len = (old_acct_len * 32 + 5) as usize;

                // assert that total number of guardians are greater than or equal to the recovery threshold
                assert!(old_acct_len - acct_len >= recovery_threshold, "too many guardians deleted");

                // Deserialize into WalletHeader from wallet program data
                let mut initial_data = WalletHeader::try_from_slice(&wallet_data[..old_data_len])?;

                // Add new guardian into deserialized struct
                for _ in 0..acct_len {
                    let guardian_info = next_account_info(account_info_iter)?;
                    let guardian_pk = guardian_info.key;

                    // check if the key to be deleted is in the data
                    assert!(initial_data.guardians.contains(guardian_pk), "the guardian to be replaced is not in the data");

                    // get index of guardian key to be deleted in the data
                    let index = initial_data.guardians.iter().position(|&k| k == *guardian_pk).unwrap();

                    // replace old with new key in the index of old key
                    initial_data.guardians.remove(index);
                    msg!("deleted guardian {:?}", guardian_pk.to_bytes());
                }

                // print all guardians
                for i in 0..initial_data.guardians.len() {
                    msg!("Guardian {}: {:?}", i, initial_data.guardians[i].to_bytes());
                }

                // Serialize struct (after adding guardians) into wallet program data
                let initial_data_len = initial_data.try_to_vec()?.len();
                msg!("data len: {}", initial_data_len);
                msg!("Serializing...");
                let mut writer = &mut wallet_data[..initial_data_len];
                initial_data.serialize(&mut writer)?;

                Ok(())
            }
            EchoInstruction::ModifyRecoveryThreshold { 
                new_threshold 
            } => {
                msg!("Instruction: ModifyRecoveryThreshold");

                let wallet_info = next_account_info(account_info_iter)?;
                let authority_info = next_account_info(account_info_iter)?;

                // find pda of wallet program that corresponds to usdc bucket for given authority
                let (wallet_pda, _) = Pubkey::find_program_address(
                    &[
                        b"bucket",
                        authority_info.key.as_ref(),
                        // usdc mint public key
                        usdc_pk.as_ref(),
                    ],
                    program_id,
                );

                if wallet_pda != *wallet_info.key {
                    return Err(ProgramError::InvalidSeeds)
                }

                assert!(new_threshold <= 10 && new_threshold > 0, "Threshold must be between 1 and 10");

                // Add the guardian data into wallet program data
                let wallet_data = &mut wallet_info.try_borrow_mut_data()?;
                wallet_data[0] = new_threshold;

                Ok(())
            }
        }
    }
}
