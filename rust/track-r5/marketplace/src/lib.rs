pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

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
}
