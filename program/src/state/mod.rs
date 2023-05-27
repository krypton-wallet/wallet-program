mod guard;
mod native_sol_transfer_guard;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

pub use self::native_sol_transfer_guard::{NativeSolTransferGuard, NativeSolTransferInterval};

/*
    32: authority pubkey
    1: recovery_threshold
    330: 33 * 10 space (32: pubkey, 1: has_signed) for MAX_GUARDIANS guardians
    32: recovery pubkey
*/
pub const MAX_GUARDIANS: u8 = 10;
pub const DATA_LEN: usize = 32 + 1 + (32 + 1) * MAX_GUARDIANS as usize + 32;
pub const PDA_SEED: &[u8] = b"profile";

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, Default)]
pub struct Guardian {
    /// Pubkey of guardian
    pub pubkey: Pubkey,
    /// flag to determine if guardian signed for recovery
    pub has_signed: bool,
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

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Guard {
    NativeSolTransfer(NativeSolTransferGuard),
}

// add an account to hold guards?
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, ShankAccount)]
pub struct GuardAccount {
    pub target: Pubkey,
    pub guard: Guard,
}
