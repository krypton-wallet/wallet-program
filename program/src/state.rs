use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
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

/// Returns associated profile PDA for data_account PubKey
pub fn get_profile_pda(datakey: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PDA_SEED, datakey.as_ref()], program_id)
}

/// Verifies if profile_data has at least recover_threshold guardian signatures
pub fn verify_recovery_state(profile_data: &ProfileHeader) -> bool {
    let num_signatures = profile_data
        .guardians
        .into_iter()
        .filter(|guardian| guardian.has_signed)
        .count();
    num_signatures >= profile_data.recovery_threshold as usize
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, ShankAccount)]
pub struct ProfileHeader {
    /// keypair Pubkey of PDA
    pub authority: Pubkey,
    /// number of guardian signatures required to sign on recovery
    pub recovery_threshold: u8,
    /// guardians
    pub guardians: [Guardian; 10],
    /// new PDA Pubkey to recover wallet into
    pub recovery: Pubkey,
}
