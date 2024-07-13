pub mod constants;
// pub mod utils;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Ed6FZS4cpdTPrqoFihR3QciyhaqS2FedHRDs46bDb2Zs");

#[program]
pub mod demo_program {

    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        instructions::create_counter(ctx)
    }

    pub fn increment_count(ctx: Context<IncrementCount>) -> Result<()> {
        instructions::increment_count(ctx)
    }
}

