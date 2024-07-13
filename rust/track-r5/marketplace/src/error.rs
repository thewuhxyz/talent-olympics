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
}