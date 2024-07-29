use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token_2022::spl_token_2022::native_mint as native_mint_2022;
use anchor_spl::token_interface::{
    approve, revoke, sync_native, transfer_checked, Approve, Revoke, SyncNative, TransferChecked,
};

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

pub fn is_native_mint(id: &Pubkey) -> bool {
    native_mint::check_id(id) || native_mint_2022::check_id(id)
}

pub fn approve_delegate<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    amount: u64,
    delegate: &A,
    to: &L,
    authority: &M,
    token_program: &P,
) -> Result<()> {
    approve(
        CpiContext::new(
            token_program.to_account_info(),
            Approve {
                to: to.to_account_info(),
                authority: authority.to_account_info(),
                delegate: delegate.to_account_info(),
            },
        ),
        amount,
    )
}

pub fn revoke_delegate<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
>(
    authority: &L,
    source: &A,
    token_program: &P,
) -> Result<()> {
    revoke(CpiContext::new(
        token_program.to_account_info(),
        Revoke {
            authority: authority.to_account_info(),
            source: source.to_account_info(),
        },
    ))
}
