use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(nonce_string: String)]
pub struct CreateAsset<'info> {
    #[account(
        mut,
        seeds = [NETWORK_SEED],
        bump
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        init,
        seeds = [ASSET_SEED, payer.key().as_ref(), nonce_string.as_bytes()],
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
    pub fn handler(&mut self, owner: Pubkey, bump: u8, metadata_uri: String) -> Result<()> {
        self.asset
            .create(self.payer.key(), owner, bump, metadata_uri)?;
        Ok(())
    }
}
