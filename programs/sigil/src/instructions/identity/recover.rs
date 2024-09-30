use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RecoverAccount<'info> {
    #[account(
        mut,
        seeds = [NETWORK_SEED],
        bump
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(mut)]
    pub identity: Box<Account<'info, Identity>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RecoverAccount<'info> {
    pub fn handler(&mut self) -> Result<()> {
        self.identity.recover(self.payer.key())?;
        Ok(())
    }
}
