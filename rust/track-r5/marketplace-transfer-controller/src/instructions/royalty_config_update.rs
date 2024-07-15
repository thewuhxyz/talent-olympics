use crate::state::MintRoyaltyConfig;

use anchor_lang::prelude::*;

use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
pub struct RoyaltyConfigUpdate<'info> {
    /// CHECK: mint account, yet to be initialized
    #[account()]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK:...
    pub service_account: Signer<'info>,

    #[account(
        mut, 
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
    )]
    pub mint_royalty_config: Account<'info, MintRoyaltyConfig>,
}

pub fn royalty_config_update(ctx: Context<RoyaltyConfigUpdate>, is_selling: bool) -> Result<()> {
    ctx.accounts.mint_royalty_config.is_selling = is_selling;
    Ok(())
}
