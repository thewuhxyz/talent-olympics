use crate::constants::*;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(
        init, 
        payer=authority, 
        space=8+Counter::INIT_SPACE,
        seeds=[COUNTER_SEEDS,authority.key().as_ref()],
        bump, 
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = anchor_lang::system_program::ID)]
    pub system_program: Program<'info, System>
}

pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
    ctx.accounts.counter.pubkey = ctx.accounts.counter.key();
    ctx.accounts.counter.authority = ctx.accounts.authority.key();
    ctx.accounts.counter.bump = ctx.bumps.counter;
    ctx.accounts.counter.count = 0;
    Ok(())
}