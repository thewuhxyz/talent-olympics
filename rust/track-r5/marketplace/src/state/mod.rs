use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceAccount {
    pub authority: Pubkey,
    pub service_mint: Pubkey,
    pub bump: u8,
}

impl ServiceAccount {
    pub fn init(&mut self, authority: Pubkey, service_mint: Pubkey, bump: u8) -> Result<()> {
        if self == &mut ServiceAccount::default() {
            return Err(error!(ErrorCode::NotAdmin));
        }
        
        self.authority = authority;
        self.service_mint = service_mint;
        self.bump = bump;
        Ok(())
    }
}

impl Default for ServiceAccount {
    fn default() -> Self {
        ServiceAccount {
            authority: Pubkey::default(),
            service_mint: Pubkey::default(),
            bump: 0,
        }
    }
}


/// Service Metadata
/// Sellable
/// name
/// price
/// description
/// additional info
/// - make it so that the user cannot remove the compulsory info. But anything else
/// 
pub struct ServiceAgreement {}