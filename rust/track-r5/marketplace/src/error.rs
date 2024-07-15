use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

    #[msg("Service Account already initialized")]
    ServiceAccountAlreadyInitialized,

    #[msg("Metadata Key does not match")]
    MetadataKeyMismatch,
    
    #[msg("Could not parse Metadata value")]
    MetadataValueParseError,
    
    #[msg("Service Account keys did not match the provided account key")]
    ServiceAccountMismatch,

    #[msg("The token is not currently transferring")]
    IsNotCurrentlyTransferring,
    
    #[msg("The token is not currently on being sold")]
    IsNotCurrentlyReselling,
    
    #[msg("The ticket is not listed")]
    IsNotListed,
    
    #[msg("Overflow occurred")]
    OverflowOccurred,
}