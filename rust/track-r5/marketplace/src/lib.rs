pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("HYLnmvP84H2xnoVq1RSduzvdBVSibs3ZyxNfK6ak8VwL");

#[program]
pub mod marketplace {

    use super::*;

    pub fn list_service(
        ctx: Context<ListService>,
        service_agreement_config: ServiceAgreementConfig,
    ) -> Result<()> {
        instructions::list_service(ctx, service_agreement_config)
    }

    pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
        instructions::buy_service(ctx)
    }

    pub fn relist_service(ctx: Context<Relist>) -> Result<()> {
        instructions::relist(ctx)
    }

    pub fn unlist_service(ctx: Context<Unlist>) -> Result<()> {
        instructions::unlist(ctx)
    }

    pub fn resell_service<'info>(ctx: Context<'_, '_, 'info, 'info, Resell<'info>>) -> Result<()> {
        instructions::resell(ctx)
    }

    pub fn royalties_init(ctx: Context<RoyaltiesInit>) -> Result<()> {
        instructions::royalties_init(ctx)
    }
}
