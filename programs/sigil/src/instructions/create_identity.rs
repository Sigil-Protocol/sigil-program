use crate::state::*;
use anchor_lang::prelude::*;
use std::str::from_utf8;

#[derive(Accounts)]
pub struct CreateIdentity<'info> {
    #[account(
        mut,
        seeds = [NETWORK_SEED],
        bump
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        init,
        seeds = [DID_METHOD, payer.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Identity>()
    )]
    pub identity: Box<Account<'info, Identity>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateIdentity<'info> {
    pub fn handler(
        &mut self,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
        bump: u8,
    ) -> Result<()> {
        let identifier_string = from_utf8(DID_METHOD).unwrap().to_string();
        let appended_string = &self.payer.key.to_string();
        let mut did_identifier = identifier_string + ":" + appended_string;

        self.identity.create(
            *self.payer.key,
            did_identifier.as_bytes().to_vec(),
            metadata_uri,
            metadata_merkle_root,
            bump,
        )?;
        Ok(())
    }
}
