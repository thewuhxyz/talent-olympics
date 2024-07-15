pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Bi2dB1dvse6p9nEDSseRC2qgnWXWFHadFSTxTjc4f5EF");

#[program]
pub mod marketplace_transfer_controller {

    use super::*;

    pub fn royalty_init(ctx: Context<RoyaltyInit>) -> Result<()> {
        instructions::royalty_init(ctx)
    }

    pub fn royalty_init_extra_metas(ctx: Context<RoyaltyInitExtraMetas>) -> Result<()> {
        instructions::royalty_init_extra_metas(ctx)
    }
    
    pub fn royalty_update(ctx: Context<RoyaltyUpdate>, is_selling: bool) -> Result<()> {
        instructions::royalty_update(ctx, is_selling)
    }

    pub fn royalties(ctx: Context<Royalties>, amount: u64) -> Result<()> {
        instructions::royalties(ctx, amount)
    }

    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        instructions::fallback(program_id, accounts, data)
    }
}
