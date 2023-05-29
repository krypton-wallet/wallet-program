use chrono::{Duration, NaiveDateTime};
use solana_program::clock::Clock;

use super::guard::Guard;
use crate::prelude::*;

#[non_exhaustive]
#[derive(Default, BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum NativeSolTransferInterval {
    #[default]
    Day,
}

impl NativeSolTransferInterval {
    fn as_duration(&self) -> Duration {
        match self {
            Self::Day => Duration::seconds(86_400),
        }
    }
}

#[derive(Debug, Default, BorshSerialize, BorshDeserialize, Clone)]
pub struct Context {
    balance_before: u64,
}

#[derive(Default, Debug, BorshSerialize, BorshDeserialize, Clone)]
pub struct NativeSolTransferGuard {
    target: Pubkey,
    transfer_amount_remaining: u64,
    transfer_limit: u64,
    transfer_interval: NativeSolTransferInterval,
    last_transferred: i64,
    context: Option<Context>,
}

impl NativeSolTransferGuard {
    pub const LENGTH: u8 = 32 //pubkey
    + 8 // u64
    + 8 // u64
    + 8 // i64
    + 1 // interval (enum)
    + 8 // last transferred
    + 8 + 1; // context

    pub fn new(target: &Pubkey, transfer_limit: u64, interval: NativeSolTransferInterval) -> Self {
        Self {
            target: target.to_owned(),
            transfer_limit,
            transfer_amount_remaining: transfer_limit,
            last_transferred: 21600,
            transfer_interval: interval,
            context: None,
        }
    }

    pub fn last_transferred(&self) -> Result<NaiveDateTime, KryptonError> {
        NaiveDateTime::from_timestamp_opt(self.last_transferred, 0)
            .ok_or(KryptonError::InvalidSysProgram)
    }
}

impl Guard for NativeSolTransferGuard {
    fn setup(&mut self, accounts: &[AccountInfo]) -> ProgramResult {
        let target = accounts
            .iter()
            .find(|a| a.key == &self.target)
            .ok_or(KryptonError::InvalidSysProgram)?;

        self.context = Some(Context {
            balance_before: target.try_lamports()?,
        });

        Ok(())
    }

    fn run(&mut self, accounts: &[AccountInfo]) -> ProgramResult {
        let Context { balance_before } =
            self.context.take().ok_or(KryptonError::InvalidSysProgram)?;
        let target = accounts
            .iter()
            .find(|a| a.key == &self.target)
            .ok_or(KryptonError::InvalidSysProgram)?;
        let desired_transfer_amount = balance_before - target.try_lamports()?;
        let date_last_transferred = self.last_transferred()?.date();
        let now = NaiveDateTime::from_timestamp_opt(Clock::get()?.unix_timestamp, 0)
            .ok_or(KryptonError::InvalidSysProgram)?;
        let today = now.date();

        let transfer_budget = if date_last_transferred == today {
            self.transfer_amount_remaining
        } else {
            self.transfer_limit
        };

        match transfer_budget.checked_sub(desired_transfer_amount) {
            Some(new_amount_remaining) => {
                self.transfer_amount_remaining = new_amount_remaining;
                self.last_transferred = now.timestamp();
                Ok(())
            }
            None => Err(KryptonError::InvalidSysProgram.into()),
        }
    }
}
