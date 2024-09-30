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

    #[msg("Unauthorized.")]
    Unauthorized,

    #[msg("Recovery Account is owner.")]
    RecoveryAccountIsOwner,

    #[msg("Recovery Account not owner.")]
    RecoveryAccountNotOwner,

    #[msg("Max Recovery Accounts reached.")]
    MaxRecoveryAccounts,

    #[msg("Recovery Account already exists.")]
    RecoveryAccountAlreadyExists,

    #[msg("Recovery Account not found.")]
    RecoveryAccountNotFound,
}
