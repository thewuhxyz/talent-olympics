use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub pubkey: Pubkey,
    pub authority: Pubkey,
    pub count: u64,
    pub bump: u8,
}
