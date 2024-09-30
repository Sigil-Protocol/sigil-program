use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddRecoveryAccount<'info> {
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

impl<'info> AddRecoveryAccount<'info> {
    pub fn handler(&mut self, recovery_account: Pubkey) -> Result<()> {
        self.identity
            .add_recovery_account(self.payer.key(), recovery_account)?;
        Ok(())
    }
}
