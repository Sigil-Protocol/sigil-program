mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("FQcz9eg5m16g62TRGoE987ZEmWuWx591WjqJEHtaB5pf");

#[program]
pub mod sigil {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
    ) -> Result<()> {
        ctx.accounts
            .handler(metadata_uri, metadata_merkle_root, ctx.bumps.identity)
    }

    pub fn update_identity(
        ctx: Context<UpdateIdentity>,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
    ) -> Result<()> {
        ctx.accounts
            .handler(metadata_uri, metadata_merkle_root, ctx.bumps.identity)
    }

    pub fn add_recovery_account(
        ctx: Context<AddRecoveryAccount>,
        recovery_account: Pubkey,
    ) -> Result<()> {
        ctx.accounts.handler(recovery_account)
    }

    pub fn remove_recovery_account(
        ctx: Context<RemoveRecoveryAccount>,
        recovery_account: Pubkey,
    ) -> Result<()> {
        ctx.accounts.handler(recovery_account)
    }

    pub fn recover(ctx: Context<RecoverAccount>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn create_asset(
        ctx: Context<CreateAsset>,
        nonce_string: String,
        owner: Pubkey,
        metadata_uri: String,
    ) -> Result<()> {
        ctx.accounts.handler(owner, ctx.bumps.asset, metadata_uri)
    }

    pub fn transfer_asset(ctx: Context<TransferAsset>, recipient: Pubkey) -> Result<()> {
        ctx.accounts.handler(recipient)
    }
}
