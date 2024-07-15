use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, PartialEq)]
pub struct MintRoyaltyConfig {
    pub provider: Pubkey,
    pub mint: Pubkey,
    pub is_selling: bool,
    pub is_initialized: bool,
}

impl MintRoyaltyConfig {
    pub fn init(&mut self, provider: Pubkey, mint: Pubkey) -> Result<()> {
        // if self == &mut ServiceAccount::default() {
        //     return Err(error!(ErrorCode::ServiceAccountAlreadyInitialized));
        // }

        self.provider = provider;
        self.mint = mint;
        self.is_selling = false;
        self.is_initialized = true;

        Ok(())
    }
}

