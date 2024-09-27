use crate::state::*;
use anchor_lang::prelude::*;
use std::str::from_utf8;

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(
        init,
        seeds = [ASSET_SEED, payer.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Asset>()
    )]
    pub asset: Box<Account<'info, Asset>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateAsset<'info> {
    pub fn handler(
        &mut self,
        issuer: Pubkey,
        owner: Pubkey,
        mint: Pubkey,
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.asset.create(issuer, owner, mint, amount, bump)?;
        Ok(())
    }
}
