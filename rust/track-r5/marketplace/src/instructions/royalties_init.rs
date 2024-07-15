use anchor_lang::prelude::*;

// use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::ServiceAccount;

#[derive(Accounts)]
pub struct RoyaltiesInit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account()]
    pub service_ticket_mint: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: ...
    pub provider: AccountInfo<'info>,

    /// CHECK: ...
    pub service_account: Account<'info, ServiceAccount>,

    /// CHECK: royalty config
    #[account(mut)]
    pub mint_royalty_config: UncheckedAccount<'info>,

    /// CHECK: royalty config
    #[account()]
    pub transfer_hook_program: UncheckedAccount<'info>,
    
    /// CHECK: royalty config
    #[account(
        mut,
        seeds=[b"signer", service_ticket_mint.key().as_ref()],
        bump,
        seeds::program=transfer_hook_program.key()
    )]
    pub delegate_signer: SystemAccount<'info>,
}

pub fn royalties_init(ctx: Context<RoyaltiesInit>) -> Result<()> {
    let system_program = &mut ctx.accounts.system_program;
    let payer = &mut ctx.accounts.payer;
    let delegate_signer = &mut ctx.accounts.delegate_signer;

    utils::system_program_transfer(890990, system_program, payer, delegate_signer)?;
    
    marketplace_transfer_controller::cpi::royalty_config_init(CpiContext::new_with_signer(
        ctx.accounts.transfer_hook_program.to_account_info(),
        marketplace_transfer_controller::cpi::accounts::RoyaltyConfigInit {
            mint_royalty_config: ctx.accounts.mint_royalty_config.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            service_account: ctx.accounts.service_account.to_account_info(),
            service_ticket_mint: ctx.accounts.service_ticket_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            // delegate_signer: ctx.accounts.delegate_signer.to_account_info()
        },
        &[&[
            b"service-account".as_ref(),
            ctx.accounts.service_ticket_mint.key().as_ref(),
            &[ctx.accounts.service_account.bump],
        ]],
    ))?;

    Ok(())
}
