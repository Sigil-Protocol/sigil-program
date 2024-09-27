use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("This amount is not enough.")]
    AmountNotEnough,

    #[msg("This stake is already running.")]
    AlreadyInitialized,

    #[msg("Invalid DID Method.")]
    InvalidDidMethod,

    #[msg("Invalid DID Identifier.")]
    InvalidDidIdentifier,

    #[msg("Invalid Metdata URI.")]
    InvalidMetadataUri,

    #[msg("Max Recovery Accounts reached.")]
    MaxRecoveryAccounts,

    #[msg("Recovery Account not found.")]
    RecoveryAccountNotFound,
}
