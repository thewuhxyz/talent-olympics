use anchor_lang::prelude::*;
use anchor_spl::{token_2022::Token2022, token_interface::{Mint, TokenAccount}};

use crate::state::{ServiceAccount, ServiceAgreement};

#[derive(Accounts)]
pub struct RoyaltiesInit<'info> {
    #[account(mut)]
    pub holder: Signer<'info>,

    #[account(
        mut,
        constraint = service_ticket_token.amount == 1,
        associated_token::token_program=token_program,
        associated_token::mint = service_ticket_mint,
        associated_token::authority = holder,
    )]
    pub service_ticket_token: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::decimals = 0,
        extensions::metadata_pointer::metadata_address = service_ticket_mint,
        extensions::group_member_pointer::member_address = service_ticket_mint,
    )]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,

    #[account(
        has_one=holder,
        constraint=service_account.mint==service_ticket_mint.key(), 
        seeds=[service_ticket_mint.key().as_ref()],
        bump
    )]
    pub service_account: Account<'info, ServiceAccount>,

    /// CHECK: service provider
    #[account(
        address=ServiceAgreement::try_from(service_ticket_mint.to_account_info())?.provider,
    )]
    pub provider: UncheckedAccount<'info>,

    /// CHECK: mint royalty config, yet to be initialized
    #[account(
        mut,
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
        seeds::program=transfer_hook_program.key(),
    )]
    pub mint_royalty_config: UncheckedAccount<'info>,

    /// CHECK: transfer_controller_program
    #[account(address=marketplace_transfer_controller::ID)]
    pub transfer_hook_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token2022>,
}

pub fn royalties_init(ctx: Context<RoyaltiesInit>) -> Result<()> {
    marketplace_transfer_controller::cpi::royalty_config_init(CpiContext::new_with_signer(
        ctx.accounts.transfer_hook_program.to_account_info(),
        marketplace_transfer_controller::cpi::accounts::RoyaltyConfigInit {
            mint_royalty_config: ctx.accounts.mint_royalty_config.to_account_info(),
            payer: ctx.accounts.holder.to_account_info(),
            service_account: ctx.accounts.service_account.to_account_info(),
            service_ticket_mint: ctx.accounts.service_ticket_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        &[&[
            ctx.accounts.service_ticket_mint.key().as_ref(),
            &[ctx.accounts.service_account.bump],
        ]],
    ))?;

    Ok(())
}
