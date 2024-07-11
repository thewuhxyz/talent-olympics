pub mod constants;
pub mod utils;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Ed6FZS4cpdTPrqoFihR3QciyhaqS2FedHRDs46bDb2Zs");

#[program]
pub mod marketplace {

    use super::*;

    pub fn list_service(ctx: Context<ListService>, args: ListServiceArgs) -> Result<()> {
        instructions::list_service(ctx, args)
    }
    
    pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
        instructions::buy_service(ctx)
    }
}

