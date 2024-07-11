use crate::constants::*;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct IncrementCount<'info> {
    #[account(
        mut, 
        has_one=authority,
        seeds=[COUNTER_SEEDS,authority.key().as_ref()],
        bump=counter.bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn increment_count(ctx: Context<IncrementCount>) -> Result<()> {
    ctx.accounts.counter.count += 1;
    Ok(())
}