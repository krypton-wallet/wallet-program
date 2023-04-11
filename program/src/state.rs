use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ProfileHeader {
    pub recovery_threshold: u8,
    pub guardians: Vec<Pubkey>,
    pub guardian_idxs: Vec<u8>,
    pub priv_scan: String,
    pub priv_spend: String,
}