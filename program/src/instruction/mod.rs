pub mod add_recovery_guardians;
pub mod initialize_recovery;
pub mod initialize_wallet;
pub mod modify_recovery_threshold;
pub mod remove_recovery_guardians;
pub mod transfer_native_sol;
pub mod transfer_token;
pub mod wrap_instruction;
pub mod add_recovery_sign;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct InitializeWalletArgs {
    pub recovery_threshold: u8,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct TransferTokenArgs {
    pub amount: u64,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct TransferNativeSOLArgs {
    pub amount: u64,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct WrapInstructionArgs {
    pub num_accounts: u8,
    pub custom_data: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct AddRecoveryGuardianArgs {
    pub num_guardians: u8,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct RemoveRecoveryGuardianArgs {
    pub num_guardians: u8,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct ModifyRecoveryThresholdArgs {
    pub new_threshold: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, ShankInstruction)]
#[rustfmt::skip]
pub enum KryptonInstruction {
    #[account(0, writable, name="profile_info", desc="PDA of Krypton Program")]
    #[account(1, signer, name="authority_info", desc="Pubkey of keypair of PDA")]
    #[account(2, name="system_program", desc="Used to create/reassign the PDA")]
    InitializeWallet(InitializeWalletArgs),

    /// This instruction transfers tokens from the wallet's ATA to the destination token account
    #[account(0, name = "profile_info", desc = "PDA of Krypton Program")]
    #[account(1, signer, name = "authority_info", desc = "Pubkey of keypair of PDA")]
    #[account(2, writable, name = "token_account_info", desc = "ATA of the PDA")]
    #[account(3, writable, name = "dest_token_account_info", desc = "Destination Token Account")]
    #[account(4, name = "token_program", desc = "Used to transfer token")]
    TransferToken(TransferTokenArgs),

    /// This instruction transfers native SOL from the wallet to the destination
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program")]
    #[account(1, signer, name = "authority_info", desc = "Pubkey of keypair of PDA")]
    #[account(2, writable, name = "destination", desc = "Destination Pubkey")]
    TransferNativeSOL(TransferNativeSOLArgs),

    /// This instruction wraps the passed in instruction and invoke_signs it using the PDA
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program")]
    #[account(1, signer, name = "authority_info", desc = "Pubkey of keypair of PDA")]
    #[account(2, name = "custom_program", desc = "Calling program of the original instruction")]
    #[account(3, name = "custom_account", desc = "Account required by original instruction")]
    WrapInstruction(WrapInstructionArgs),

    /// This instruction adds a Pubkey that will act as a guardian during recovery of the wallet
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program")]
    #[account(1, signer, name = "authority_info", desc = "Pubkey of keypair of PDA")]
    #[account(2, name = "guardian", desc = "Pubkey that will act as guardian to recover profile_info")]
    AddRecoveryGuardians(AddRecoveryGuardianArgs),

    /// This instruction removes a Pubkey that will act as a guardian during recovery of the wallet
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program")]
    #[account(1, signer, name = "authority_info", desc = "Pubkey of keypair of PDA")]
    #[account(2, name = "guardian", desc = "Pubkey that will act as guardian to recover profile_info")]
    RemoveRecoveryGuardians(RemoveRecoveryGuardianArgs),

    /// This instruction modifies the number of guardian signatures required to recover the wallet
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program")]
    #[account(1, signer, name = "authority_info", desc = "Pubkey of keypair of PDA")]
    ModifyRecoveryThreshold(ModifyRecoveryThresholdArgs),

    /// This instruction initializes the recovery of the wallet into the new PDA
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program to be recovered")]
    #[account(1, name = "authority_info", desc = "Pubkey of keypair of PDA to be recovered")]
    #[account(2, name = "new_profile_info", desc = "PDA to be recovered into")]
    #[account(3, signer, name = "new_authority_info", desc = "Pubkey of the keypair to be recovered into")]
    InitializeRecovery,

    /// This instruction adds a guardian's signature for the recovery of the wallet into the new PDA
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program to be recovered")]
    #[account(1, name = "authority_info", desc = "Pubkey of keypair of PDA to be recovered")]
    #[account(2, name = "new_profile_info", desc = "PDA to be recovered into")]
    #[account(3, name = "new_authority_info", desc = "Pubkey of the keypair to be recovered into")]
    #[account(4, signer, name = "guardian_info", desc = "Pubkey of recovery guardian")]
    AddRecoverySign,

    /// This instruction recovers the wallet into the new PDA provided there are at least `recovery_threshold`
    #[account(0, name = "profile_info", desc = "PDA of Krypton Program to be recovered")]
    #[account(1, name = "authority_info", desc = "Pubkey of keypair of PDA to be recovered")]
    #[account(2, writable, name = "new_profile_info", desc = "PDA to be recovered into")]
    #[account(3, signer, name = "new_authority_info", desc = "Pubkey of the keypair to be recovered into")]
    RecoverWallet,

    /// This instruction recovers the wallet into the new PDA provided there are at least `recovery_threshold`
    /// guardian signatures for the recovery
    #[account(0, name = "profile_info", desc = "PDA of Krypton Program to be recovered")]
    #[account(1, name = "authority_info", desc = "Pubkey of keypair of PDA to be recovered")]
    #[account(2, name = "new_profile_info", desc = "PDA to be recovered into")]
    #[account(3, signer, name = "new_authority_info", desc = "Pubkey of the keypair to be recovered into")]
    #[account(4, writable, name = "old_token_account_info", desc = "ATA of the PDA to be recovered")]
    #[account(5, writable, name = "new_token_account_info", desc = "ATA of the PDA to be recovered into")]
    #[account(6, name = "token_program", desc = "Used to transfer token")]
    RecoverToken,

    /// This instruction recovers all the native SOL from the old wallet into the new PDA provided there
    #[account(0, writable, name = "profile_info", desc = "PDA of Krypton Program to be recovered")]
    #[account(1, name = "authority_info", desc = "Pubkey of keypair of PDA to be recovered")]
    #[account(2, writable, name = "new_profile_info", desc = "PDA to be recovered into")]
    #[account(3, signer, name = "new_authority_info", desc = "Pubkey of the keypair to be recovered into")]
    RecoverNativeSOL,
}
