use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, TokenInterface};

#[derive(Accounts)]
pub struct RoyaltyInit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account()]
    pub service_ticket_mint: UncheckedAccount<'info>,

    /// CHECK: Program ID
    #[account()]
    pub wsol_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,

    /// CHECK: ...
    pub provider: AccountInfo<'info>,

    /// CHECK: ...
    pub service_account: AccountInfo<'info>,

    /// CHECK: royalty config
    #[account(mut)]
    pub mint_royalty_config: UncheckedAccount<'info>,

    /// CHECK: ...
    #[account(mut)]
    pub mint_royalty_wsol_token_account: UncheckedAccount<'info>,

    /// CHECK: ...
    pub token_program_classic: Interface<'info, TokenInterface>,

    /// CHECK: ...
    pub associated_token_program: UncheckedAccount<'info>,

    /// CHECK
    pub transfer_hook_program: UncheckedAccount<'info>,
}

pub fn royalty_init(ctx: Context<RoyaltyInit>) -> Result<()> {
    marketplace_transfer_controller::cpi::royalty_init(CpiContext::new(
        ctx.accounts.transfer_hook_program.to_account_info(),
        marketplace_transfer_controller::cpi::accounts::RoyaltyInit {
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            mint_royalty_config: ctx.accounts.mint_royalty_config.to_account_info(),
            mint_royalty_wsol_token_account: ctx
                .accounts
                .mint_royalty_wsol_token_account
                .to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            provider: ctx.accounts.provider.to_account_info(),
            service_account: ctx.accounts.service_account.to_account_info(),
            service_ticket_mint: ctx.accounts.service_ticket_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program_classic: ctx.accounts.token_program_classic.to_account_info(),
            wsol_mint: ctx.accounts.wsol_mint.to_account_info(),
        },
    ))?;

    Ok(())
}
