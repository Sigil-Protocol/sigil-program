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
}
