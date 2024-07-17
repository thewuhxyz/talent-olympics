use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

// #[account]
#[derive(InitSpace, Clone, BorshDeserialize)]
pub struct MintRoyaltyConfig {
    pub mint: Pubkey,
    pub delegate_bump: u8,
    pub is_selling: bool,
    pub is_enabled: bool,
    pub is_initialized: bool,
}

// impl MintRoyaltyConfig {
//     pub fn init(&mut self, mint: Pubkey) -> Result<()> {
//         self.mint = mint;
//         self.is_selling = false;
//         self.is_enabled = true;
//         self.is_initialized = true;
//         Ok(())
//     }
// }

