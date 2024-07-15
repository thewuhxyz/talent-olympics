use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, PartialEq)]
pub struct MintRoyaltyConfig {
    pub mint: Pubkey,
    pub is_selling: bool,
    pub is_enabled: bool,
    pub is_initialized: bool,
}

impl MintRoyaltyConfig {
    pub fn init(&mut self, mint: Pubkey) -> Result<()> {
        self.mint = mint;
        self.is_selling = false;
        self.is_enabled = true;
        self.is_initialized = true;
        Ok(())
    }
}

