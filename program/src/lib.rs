pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL");

pub mod prelude {
    pub use crate::{
        error::KryptonError,
        state::{get_profile_pda, Guardian, ProfileHeader, DATA_LEN, MAX_GUARDIANS, PDA_SEED},
    };
    pub use borsh::{BorshDeserialize, BorshSerialize};

    pub use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        pubkey::Pubkey,
    };
    pub use solana_program::{
        rent::Rent,
        system_instruction::{assign, create_account},
        sysvar::Sysvar,
    };
}
