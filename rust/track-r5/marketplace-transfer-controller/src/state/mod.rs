use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, PartialEq)]
pub struct MintRoyaltyConfig {
    pub mint: Pubkey,
    pub delegate_bump: u8,
    pub is_selling: bool,
    pub is_enabled: bool,
    pub is_initialized: bool,
}

impl MintRoyaltyConfig {
    pub fn init(&mut self, mint: Pubkey, delegate_bump: u8) -> Result<()> {
        self.mint = mint;
        self.delegate_bump = delegate_bump;
        self.is_selling = false;
        self.is_enabled = true;
        self.is_initialized = true;
        Ok(())
    }

    fn some () {
        Self::try_deserialize(buf)
    }
}
