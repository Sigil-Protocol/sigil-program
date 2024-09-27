use crate::errors;
use anchor_lang::prelude::*;
use solana_program::pubkey;
use std::str::from_utf8;

pub const NETWORK_SEED: &[u8] = b"network";
pub const IDENTITY_SEED: &[u8] = b"identity";
pub const RECOVERY_SEED: &[u8] = b"recovery";
pub const DID_METHOD: &[u8] = b"did:sigil";
pub const ASSET_SEED: &[u8] = b"asset";

const MAX_DID_METHOD_LENGTH: usize = 10;
const MAX_IDENTIFIER_LENGTH: usize = 64;
const MAX_URI_LENGTH: usize = 200;

pub const MAX_SIZE: usize =
    32 + 4 + MAX_DID_METHOD_LENGTH + 4 + MAX_IDENTIFIER_LENGTH + 4 + MAX_URI_LENGTH + 8 + 8;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum LoanStatus {
    Offered,
    Accepted,
    Repaid,
    Defaulted,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum InvoiceStatus {
    Pending,
    Paid,
    Overdue,
}

#[account]
pub struct Network {
    pub admin: Pubkey,
    pub total_identities: u64,
}

#[account]
pub struct Identity {
    pub owner: Pubkey,
    pub did_method: String,
    pub did_identifier: Vec<u8>,
    pub recovery_accounts: Vec<Pubkey>,
    pub metadata_uri: String,
    pub metadata_merkle_root: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[account]
pub struct Asset {
    pub issuer: Pubkey,
    pub owner: Pubkey,
    pub attestor: Option<Pubkey>,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

#[account]
pub struct Invoice {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub due_at: i64,
    pub daily_penalty_rate: u8,
    pub status: InvoiceStatus,
}

#[account]
pub struct Loan {
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub amount: u64,
    pub interest_rate: u8,
    pub duration_days: u64,
    pub status: LoanStatus,
}

// Implementations
impl Identity {
    pub fn create(
        &mut self,
        owner: Pubkey,
        did_identifier: Vec<u8>,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
        bump: u8,
    ) -> Result<()> {
        if metadata_uri.len() > MAX_URI_LENGTH {
            return Err(errors::ErrorCode::InvalidMetadataUri.into());
        }

        self.owner = owner;
        self.did_method = from_utf8(DID_METHOD).unwrap().to_string();
        self.did_identifier = did_identifier;
        self.metadata_uri = metadata_uri;
        self.metadata_merkle_root = metadata_merkle_root;
        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        self.bump = bump;

        Ok(())
    }

    pub fn update(
        &mut self,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
        bump: u8,
    ) -> Result<()> {
        if metadata_uri.len() > MAX_URI_LENGTH {
            return Err(errors::ErrorCode::InvalidMetadataUri.into());
        }

        self.metadata_uri = metadata_uri;
        self.metadata_merkle_root = metadata_merkle_root;
        self.updated_at = Clock::get()?.unix_timestamp;
        self.bump = bump;

        Ok(())
    }

    pub fn add_recovery_account(&mut self, recovery_account: Pubkey) -> Result<()> {
        if self.recovery_accounts.len() >= 3 {
            return Err(errors::ErrorCode::MaxRecoveryAccounts.into());
        }

        self.recovery_accounts.push(recovery_account);
        Ok(())
    }

    pub fn remove_recovery_account(&mut self, recovery_account: Pubkey) -> Result<()> {
        let index = self
            .recovery_accounts
            .iter()
            .position(|&r| r == recovery_account)
            .ok_or(errors::ErrorCode::RecoveryAccountNotFound)?;

        self.recovery_accounts.remove(index);
        Ok(())
    }

    pub fn recover(&mut self, new_owner: Pubkey) -> Result<()> {
        if !self.recovery_accounts.contains(&new_owner) {
            return Err(errors::ErrorCode::RecoveryAccountNotFound.into());
        }

        self.owner = new_owner;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.owner = Pubkey::new_unique();
        self.did_method = "".to_string();
        self.did_identifier = vec![];
        self.recovery_accounts = vec![];
        self.metadata_uri = "".to_string();
        self.metadata_merkle_root = vec![];
        self.created_at = 0;
        self.updated_at = 0;
        self.bump = 0;

        Ok(())
    }
}

impl Asset {
    pub fn create(
        &mut self,
        issuer: Pubkey,
        owner: Pubkey,
        mint: Pubkey,
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.issuer = issuer;
        self.owner = owner;
        self.attestor = None;
        self.mint = mint;
        self.amount = amount;
        self.bump = bump;

        Ok(())
    }

    pub fn transfer(&mut self, new_owner: Pubkey) -> Result<()> {
        self.owner = new_owner;
        Ok(())
    }

    pub fn attest(&mut self, new_attestor: Pubkey) -> Result<()> {
        self.attestor = Some(new_attestor);
        Ok(())
    }

    pub fn burn(&mut self, amount: u64) -> Result<()> {
        if self.amount < amount {
            return Err(errors::ErrorCode::AmountNotEnough.into());
        }

        self.amount -= amount;
        Ok(())
    }

    pub fn mint(&mut self, amount: u64) -> Result<()> {
        self.amount += amount;
        Ok(())
    }
}
