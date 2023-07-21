use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, FromPrimitive, PartialEq)]
pub enum KryptonError {
    #[error("Account should be writeable")]
    NotWriteable,
    #[error("Account should not have 0 length data")]
    NoAccountLength,
    #[error("Account should not have non-zero data")]
    NonZeroData,
    #[error("Account should be signer")]
    NotSigner,
    #[error("Account should be valid system program")]
    InvalidSysProgram,
    #[error("There are too many guardians")]
    TooManyGuardians,
    #[error("There are too few guardians passed in")]
    NotEnoughGuardians,
    #[error("Specified amount of accounts are not passed in")]
    NotEnoughAccounts,
    #[error("The Guardian provided is not in the data")]
    GuardianNotFound,
    #[error("There are not enough guardian signatures to recover")]
    MissingGuardianSignatures,
    #[error("Recovery Threshold must be between 1 to 10")]
    InvalidRecoveryThreshold,
    #[error("The pubkey is not authorized to act on behalf of the wallet")]
    InvalidAuthority,
    #[error("The pubkey is not authorized to recover the wallet")]
    NotAuthorizedToRecover,
    #[error("Required recovered accounts are not passed in")]
    MissingRecoveredAccounts,
    #[error("There is insufficient SOL to transfer")]
    InsufficientFundsForTransaction,
    #[error("Operation overflowed")]
    Overflow,
    #[error("Invalid Account Address")]
    InvalidAccountAddress,
    #[error("Invalid date time")]
    InvalidDateTime,
    #[error("Target account not found")]
    TargetAccountNotFound,
    #[error("Invalid target for guard")]
    InvalidGuardTarget,
    #[error("Guard Context not found")]
    GuardContextNotFound,
    #[error("Arithmetic Overflow")]
    ArithmeticOverflow,
}

impl From<KryptonError> for ProgramError {
    fn from(e: KryptonError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
