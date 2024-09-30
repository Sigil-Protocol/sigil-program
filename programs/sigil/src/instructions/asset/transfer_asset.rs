use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferAsset<'info> {
    #[account(mut)]
    pub asset: Box<Account<'info, Asset>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferAsset<'info> {
    pub fn handler(&mut self, recipient: Pubkey) -> Result<()> {
        let payer_key = self.payer.key();
        let asset_owner = self.asset.owner;

        self.asset.transfer(payer_key, asset_owner, recipient)?;
        Ok(())
    }
}
