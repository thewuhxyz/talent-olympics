use crate::state::MintRoyaltyConfig;

use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount};

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

    #[account(
        init, 
        payer=payer,
        space=8+MintRoyaltyConfig::INIT_SPACE,
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
    )]
    pub mint_royalty_config: Account<'info, MintRoyaltyConfig>,

    /// CHECK: ...
    #[account(
        init,
        payer=payer,
        associated_token::token_program=token_program_classic,
        associated_token::authority=mint_royalty_config,
        associated_token::mint=wsol_mint,
    )]
    pub mint_royalty_wsol_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: ...
    pub token_program_classic: Interface<'info, TokenInterface>,
    
    /// CHECK: ...
    pub associated_token_program: UncheckedAccount<'info>,
}

pub fn royalty_init(ctx: Context<RoyaltyInit>) -> Result<()> {
    let service_ticket_mint = &mut ctx.accounts.service_ticket_mint;
    let mint_royalty_config = &mut ctx.accounts.mint_royalty_config;
    let provider = &mut ctx.accounts.provider;

    mint_royalty_config.init(provider.key(), service_ticket_mint.key())?;

    Ok(())
}

