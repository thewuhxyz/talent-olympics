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

    pub fn royalty_config_init(ctx: Context<RoyaltyConfigInit>) -> Result<()> {
        instructions::royalty_config_init(ctx)
    }

    pub fn royalty_config_update(
        ctx: Context<RoyaltyConfigUpdate>,
        is_selling: bool,
    ) -> Result<()> {
        instructions::royalty_config_update(ctx, is_selling)
    }

    pub fn transfer_control_init(ctx: Context<TransferControlInit>) -> Result<()> {
        instructions::transfer_control_init(ctx)
    }

    pub fn transfer_control(ctx: Context<TransferControl>, amount: u64) -> Result<()> {
        instructions::transfer_control(ctx, amount)
    }

    pub fn transfer_control_fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        instructions::transfer_control_fallback(program_id, accounts, data)
    }
}

