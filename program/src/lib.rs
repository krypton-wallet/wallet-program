pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("2aJqX3GKRPAsfByeMkL7y9SqAGmCQEnakbuHJBdxGaDL");

pub mod prelude {
    pub use crate::{
        error::KryptonError,
        state::{
            get_profile_pda, verify_recovery_state, ProfileHeader, UserProfile, MAX_GUARDIANS,
            PDA_SEED, PROFILE_HEADER_LEN,
        },
    };
    pub use borsh::{BorshDeserialize, BorshSerialize};

    pub use spl_token::state::Account as TokenAccount;

    pub use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    pub use solana_program::{
        rent::Rent,
        system_instruction::{assign, create_account},
        sysvar::Sysvar,
    };
}
