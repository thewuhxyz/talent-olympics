use crate::state::MintRoyaltyConfig;

use anchor_lang::prelude::*;

use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
pub struct RoyaltyConfigInit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Program ID
    #[account()]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK: ...
    #[account(signer)]
    pub service_account: AccountInfo<'info>,

    #[account(
        init, 
        payer=payer,
        space=8+MintRoyaltyConfig::INIT_SPACE,
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
    )]
    pub mint_royalty_config: Account<'info, MintRoyaltyConfig>,

    #[account(
        mut,
        seeds=[b"signer".as_ref(),service_ticket_mint.key().as_ref()],
        bump,
    )]
    pub delegate_signer: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn royalty_config_init(ctx: Context<RoyaltyConfigInit>) -> Result<()> {
    let service_ticket_mint = &mut ctx.accounts.service_ticket_mint;
    let mint_royalty_config = &mut ctx.accounts.mint_royalty_config;
    // let system_program = &mut ctx.accounts.system_program;
    // let payer = &mut ctx.accounts.payer;
    // let delegate_signer = &mut ctx.accounts.delegate_signer;

    // utils::system_program_transfer(890990, system_program, payer, delegate_signer)?;

    mint_royalty_config.init(service_ticket_mint.key(), ctx.bumps.delegate_signer)?;

    Ok(())
}

