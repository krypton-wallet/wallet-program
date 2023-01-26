use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct WalletHeader {
    pub recovery_threshold: u8,
    pub guardians: Vec<Pubkey>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct VendingMachineBufferHeader {
    // TODO
}
