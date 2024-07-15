use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token_interface::{sync_native, transfer_checked, SyncNative, TransferChecked};

pub fn system_program_transfer<
    'info,
    S: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
>(
    amount: u64,
    system_program: &S,
    from: &A,
    to: &L,
) -> Result<()> {
    if amount > 0 {
        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
            ),
            amount,
        )
    } else {
        Ok(())
    }
}

pub fn token_transfer_checked_transfer<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
    R: ToAccountInfo<'info>,
>(
    amount: u64,
    decimals: u8,
    from: &A,
    to: &R,
    mint: &M,
    authority: &L,
    token_program: &P,
) -> Result<()> {
    if amount > 0 {
        transfer_checked(
            CpiContext::new(
                token_program.to_account_info(),
                TransferChecked {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    mint: mint.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
            decimals,
        )
    } else {
        Ok(())
    }
}

pub fn token_sync_native<'info, P: ToAccountInfo<'info>, L: ToAccountInfo<'info>>(
    account: &L,
    token_program: &P,
) -> Result<()> {
    sync_native(CpiContext::new(
        token_program.to_account_info(),
        SyncNative {
            account: account.to_account_info(),
        },
    ))
}
