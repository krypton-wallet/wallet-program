use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/*
    32: authority pubkey
    1: recovery_threshold
    4: size of vector of guardians
    340: 34 * 10 space (32: pubkey, 1: shard_idx, 1: has_signed) for 10 guardians
    72: 36 (4 + 32) * 2 space for 2 encrypted keys
    32: recovery pubkey
*/
pub const DATA_LEN: usize = 32 + 1 + 4 + (32 + 1 + 1) * 10 + (4 + 32) * 2 + 32;
pub const MAX_GUARDIANS: u8 = 10;
pub const PDA_SEED: &[u8] = b"profile";

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy)]
pub struct Guardian {
    /// Pubkey of guardian
    pub pubkey: Pubkey,
    /// shard idx assigned to guardian
    pub shard_idx: u8,
    /// flag to determine if guardian signed for recovery
    pub has_signed: bool,
}

impl Default for Guardian {
    fn default() -> Self {
        Self {
            pubkey: Pubkey::default(),
            shard_idx: MAX_GUARDIANS + 1,
            has_signed: false,
        }
    }
}

/// verifies if profile_data has at least recover_threshold guardian signatures
pub fn verify_recovery_state(profile_data: &ProfileHeader) -> bool {
    let num_signatures = profile_data
        .guardians
        .into_iter()
        .filter(|guardian| guardian.has_signed)
        .count();
    num_signatures >= profile_data.recovery_threshold as usize
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProfileHeader {
    /// keypair Pubkey of PDA
    pub authority: Pubkey,
    /// number of guardian signatures required to sign on recovery
    pub recovery_threshold: u8,
    /// guardians
    pub guardians: [Guardian; MAX_GUARDIANS as usize],
    pub priv_scan: Vec<u8>,
    pub priv_spend: Vec<u8>,
    /// new PDA Pubkey to recover wallet into
    pub recovery: Pubkey,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct InitializeWalletArgs {
    pub recovery_threshold: u8,
    pub priv_scan: Vec<u8>,
    pub priv_spend: Vec<u8>,
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
pub struct UpdateSecretArgs {
    pub priv_scan: Vec<u8>,
    pub priv_spend: Vec<u8>,
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

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct AddRecoverySignArgs {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct RecoverWalletArgs {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct RecoverTokenArgs {}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct RecoverNativeSOLArgs {}
