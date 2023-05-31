mod guard;
mod native_sol_transfer_guard;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};

pub const MAX_GUARDIANS: u8 = 10;
/*
    32: seed pubkey
    32: authority pubkey
*/
pub const PROFILE_HEADER_LEN: usize = 32 + 32;
pub const PDA_SEED: &[u8] = b"profile";

/// Returns associated profile PDA for data_account PubKey
pub fn get_profile_pda(datakey: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PDA_SEED, datakey.as_ref()], program_id)
}

/// Verifies if profile_data has at least recover_threshold guardian signatures
pub fn verify_recovery_state(profile_data: &UserProfile) -> bool {
    let num_signatures = profile_data
        .guardians
        .values()
        .filter(|&has_signed| *has_signed)
        .count();
    num_signatures >= profile_data.recovery_threshold as usize
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, ShankAccount)]
pub struct UserProfile {
    /// keypair Pubkey seed of PDA
    pub seed: Pubkey,
    /// authority keypair Pubkey
    pub authority: Pubkey,
    /// number of guardian signatures required to sign on recovery
    pub recovery_threshold: u8,
    /// guardians
    pub guardians: HashMap<Pubkey, bool>,
    /// new PDA Pubkey to recover wallet into
    pub recovery: Pubkey,
    /// recovered PDA Pubkeys
    pub recovered: HashSet<Pubkey>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, ShankAccount)]
pub struct ProfileHeader {
    /// keypair Pubkey seed of PDA
    pub seed: Pubkey,
    /// authority keypair Pubkey
    pub authority: Pubkey,
}
