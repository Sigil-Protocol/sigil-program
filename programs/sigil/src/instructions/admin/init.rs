use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        seeds = [NETWORK_SEED],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Network>()
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn handler(&mut self) -> Result<()> {
        self.network.admin = *self.payer.key;
        self.network.total_identities = 0;
        Ok(())
    }
}
