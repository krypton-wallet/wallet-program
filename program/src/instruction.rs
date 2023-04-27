use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::{
    AddRecoveryGuardianArgs, AddRecoverySignArgs, InitializeWalletArgs,
    ModifyRecoveryThresholdArgs, RecoverNativeSOLArgs, RecoverTokenArgs,
    RemoveRecoveryGuardianArgs, TransferNativeSOLArgs, TransferTokenArgs, UpdateSecretArgs,
    WrapInstructionArgs,
};

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum KryptonInstruction {
    /// This instruction initializes the Krypton wallet
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    /// | 2     | ❌       | ❌     | `system_program`: Used to create/reassign the PDA                        |
    InitializeWallet(InitializeWalletArgs),
    /// This instruction transfers tokens from the wallet's ATA to the destination token account
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ❌       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    /// | 2     | ✅       | ❌     | `token_account_info`: ATA of the PDA                                     |
    /// | 3     | ✅       | ❌     | `dest_token_account_info`: Destination Token Account                     |
    /// | 4     | ❌       | ❌     | `token_program`: Used to transfer token                                  |
    TransferToken(TransferTokenArgs),
    /// This instruction transfers native SOL from the wallet to the destination
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    /// | 2     | ✅       | ❌     | `destination`:  Destination Pubkey                                       |
    TransferNativeSOL(TransferNativeSOLArgs),
    /// This instruction wraps the passed in instruction and invoke_signs it using the PDA
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    /// | 2     | ❌       | ❌     | `custom_program`: Calling program of the original instruction            |
    /// | 3..   | ~        | ~      | `custom_account`: Account required by original instruction               |
    WrapInstruction(WrapInstructionArgs),
    /// This instruction updates the priv_scan and priv_spend encrypted key
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    UpdateSecret(UpdateSecretArgs),
    /// This instruction adds a Pubkey that will act as a guardian during recovery of the wallet
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    /// | 2..   | ❌       | ❌     | `guardian`: Pubkey that will act as guardian to recover profile_info     |
    AddRecoveryGuardians(AddRecoveryGuardianArgs),
    /// This instruction removes a Pubkey that will act as a guardian during recovery of the wallet
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                  |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                               |
    /// | 2..   | ❌       | ❌     | `guardian`: Pubkey that will act as guardian to recover profile_info     |
    RemoveRecoveryGuardians(RemoveRecoveryGuardianArgs),
    /// This instruction modifies the number of guardian signatures required to recover the wallet
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program                                    |
    /// | 1     | ❌       | ✅     | `authority_info`: Pubkey of keypair of PDA                                 |
    ModifyRecoveryThreshold(ModifyRecoveryThresholdArgs),
    /// This instruction adds a guardian's signature for the recovery of the wallet into the new PDA
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// | ----- | -------- | ------ | ------------------------------------------------------------------------ |
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program to be recovered                    |
    /// | 1     | ❌       | ❌     | `authority_info`: Pubkey of keypair of PDA to be recovered                 |
    /// | 2     | ❌       | ❌     | `new_profile_info`: PDA to be recovered into                               |
    /// | 3     | ❌       | ✅     | `new_authority_info`: Pubkey of the keypair to be recovered into           |
    /// | 4     | ❌       | ✅     | `guardian_info`: Pubkey of recovery guardian                               |
    AddRecoverySign(AddRecoverySignArgs),
    /// This instruction recovers the wallet into the new PDA provided there are at least `recovery_threshold`
    /// guardian signatures for the recovery
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ❌       | ❌     | `profile_info`: PDA of Krypton Program to be recovered                  |
    /// | 1     | ❌       | ❌     | `authority_info`: Pubkey of keypair of PDA to be recovered               |
    /// | 2     | ✅       | ❌     | `new_profile_info`: PDA to be recovered into                             |
    /// | 3     | ❌       | ✅     | `new_authority_info`: Pubkey of the keypair to be recovered into         |
    RecoverWallet(RecoverTokenArgs),
    /// This instruction recovers the wallet into the new PDA provided there are at least `recovery_threshold`
    /// guardian signatures for the recovery
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ❌       | ❌     | `profile_info`: PDA of Krypton Program to be recovered                  |
    /// | 1     | ❌       | ❌     | `authority_info`: Pubkey of keypair of PDA to be recovered               |
    /// | 2     | ❌       | ❌     | `new_profile_info`: PDA to be recovered into                             |
    /// | 3     | ❌       | ✅     | `new_authority_info`: Pubkey of the keypair to be recovered into         |
    /// | 4     | ✅       | ❌     | `old_token_account_info`: ATA of the PDA to be recovered                 |
    /// | 5     | ✅       | ❌     | `new_token_account_info`: ATA of the PDA to be recovered into            |
    /// | 6     | ❌       | ❌     | `token_program`: Used to transfer token                                  |
    RecoverToken(RecoverTokenArgs),
    /// This instruction recovers all the native SOL from the old wallet into the new PDA provided there
    /// are at least `recovery_threshold`
    /// guardian signatures for the recovery
    ///
    /// Accounts:
    ///
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | `profile_info`: PDA of Krypton Program to be recovered                  |
    /// | 1     | ❌       | ❌     | `authority_info`: Pubkey of keypair of PDA to be recovered               |
    /// | 2     | ✅       | ❌     | `new_profile_info`: PDA to be recovered into                             |
    /// | 3     | ❌       | ✅     | `new_authority_info`: Pubkey of the keypair to be recovered into         |
    RecoverNativeSOL(RecoverNativeSOLArgs),
}
