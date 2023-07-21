use super::TransferNativeSOLArgs;
use crate::state::guard::GuardTrait;
use crate::state::Guard::NativeSolTransfer;
use crate::{prelude::*, state::GuardAccount};

pub fn process_transfer_native_sol(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferNativeSOLArgs,
) -> ProgramResult {
    let mut account_info_iter = accounts.iter();

    let profile_info = next_account_info(&mut account_info_iter)?;
    let authority_info = next_account_info(&mut account_info_iter)?;
    let dest = next_account_info(&mut account_info_iter)?;
    let guard_info = if accounts.len() > 3 {
        Some(next_account_info(&mut account_info_iter)?)
    } else {
        None
    };

    // ensure authority_info is signer
    if !authority_info.is_signer {
        return Err(KryptonError::NotSigner.into());
    }

    // ensure profile_info is writable
    if !profile_info.is_writable {
        return Err(KryptonError::NotWriteable.into());
    }

    let profile_data =
        ProfileHeader::try_from_slice(&profile_info.try_borrow_data()?[..PROFILE_HEADER_LEN])?;

    // ensure authority_info is valid
    if profile_data.authority != *authority_info.key {
        return Err(KryptonError::InvalidAuthority.into());
    }

    // ensure guard_info is valid
    if let Some(guard_account) = guard_info {
        let guard_seeds = &[b"guard", profile_info.key.as_ref()];
        assert_derivation(
            &crate::id(),
            guard_account,
            guard_seeds,
            KryptonError::InvalidAccountAddress,
        )?;
    }

    msg!("account checks complete");

    // ensure there is enough SOL to transfer
    if **profile_info.try_borrow_lamports()? < args.amount {
        return Err(ProgramError::InsufficientFunds);
    }

    if let Some(guard_info) = guard_info {
        let mut guard_data = GuardAccount::try_from_slice(&guard_info.try_borrow_data()?)?;

        // ensure guard target is valid
        if guard_data.target != *profile_info.key {
            return Err(KryptonError::InvalidGuardTarget.into());
        }

        // extract the guard and call setup()
        if let NativeSolTransfer(ref mut sol_guard) = guard_data.guard {
            sol_guard.setup(accounts)?;
        }

        // if not enough lamports or space in guard_info, transfer lamports and realloc
        let guard_data_len = guard_data.try_to_vec()?.len();
        if guard_data_len > guard_info.data_len() {
            let rent_exempt_amount = Rent::get()?.minimum_balance(guard_data_len);
            let lamports_diff = rent_exempt_amount - guard_info.lamports();
            **profile_info.try_borrow_mut_lamports()? -= lamports_diff;
            **guard_info.try_borrow_mut_lamports()? += lamports_diff;

            guard_info.realloc(guard_data_len, false)?;
        }

        // write updated data to guard_info
        guard_data.serialize(&mut &mut guard_info.try_borrow_mut_data()?[..guard_data_len])?;

        msg!("setup done");
    }

    // debit profile_info and credit dest
    **profile_info.try_borrow_mut_lamports()? -= args.amount;
    **dest.try_borrow_mut_lamports()? += args.amount;

    if let Some(guard_info) = guard_info {
        let mut guard_data = GuardAccount::try_from_slice(&guard_info.try_borrow_data()?)?;

        // extract the guard and call run()
        if let NativeSolTransfer(ref mut sol_guard) = guard_data.guard {
            sol_guard.run(accounts)?;
        }

        let guard_data_len = guard_data.try_to_vec()?.len();
        if guard_data_len < guard_info.data_len() {
            guard_info.realloc(guard_data_len, false)?;
        }
        guard_data.serialize(&mut &mut guard_info.try_borrow_mut_data()?[..guard_data_len])?;

        msg!("run done");
    }

    msg!("amount: {}", args.amount);

    Ok(())
}
