use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    #[account(
        mut,
        seeds = [NETWORK_SEED],
        bump
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        mut,
        seeds = [DID_METHOD, payer.key().as_ref()],
        bump,
    )]
    pub identity: Box<Account<'info, Identity>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateIdentity<'info> {
    pub fn handler(
        &mut self,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
        bump: u8,
    ) -> Result<()> {
        self.identity
            .update(self.payer.key(), metadata_uri, metadata_merkle_root, bump)?;
        Ok(())
    }
}
