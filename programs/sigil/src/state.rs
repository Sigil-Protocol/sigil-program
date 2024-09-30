use crate::errors;
use anchor_lang::prelude::*;
use solana_program::pubkey;
use std::str::from_utf8;

pub const NETWORK_SEED: &[u8] = b"sigil";
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
    pub bump: u8,
    pub created_at: i64,
    pub updated_at: i64,
}

#[account]
pub struct Asset {
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub attestor: Option<Pubkey>,
    pub metadata_uri: String,
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
        self.recovery_accounts = vec![];
        self.metadata_uri = metadata_uri;
        self.metadata_merkle_root = metadata_merkle_root;
        self.created_at = Clock::get()?.unix_timestamp;
        self.updated_at = Clock::get()?.unix_timestamp;
        self.bump = bump;

        Ok(())
    }

    pub fn update(
        &mut self,
        payer: Pubkey,
        metadata_uri: String,
        metadata_merkle_root: Vec<u8>,
        bump: u8,
    ) -> Result<()> {
        if payer != self.owner {
            return Err(errors::ErrorCode::Unauthorized.into());
        }

        if metadata_uri.len() > MAX_URI_LENGTH {
            return Err(errors::ErrorCode::InvalidMetadataUri.into());
        }

        self.metadata_uri = metadata_uri;
        self.metadata_merkle_root = metadata_merkle_root;
        self.updated_at = Clock::get()?.unix_timestamp;
        self.bump = bump;

        Ok(())
    }

    pub fn add_recovery_account(&mut self, payer: Pubkey, recovery_account: Pubkey) -> Result<()> {
        if payer != self.owner {
            return Err(errors::ErrorCode::Unauthorized.into());
        }

        if self.owner == recovery_account {
            return Err(errors::ErrorCode::RecoveryAccountIsOwner.into());
        }

        if self.recovery_accounts.len() >= 3 {
            return Err(errors::ErrorCode::MaxRecoveryAccounts.into());
        }

        if self.recovery_accounts.contains(&recovery_account) {
            return Err(errors::ErrorCode::RecoveryAccountAlreadyExists.into());
        }

        self.recovery_accounts.push(recovery_account);
        Ok(())
    }

    pub fn remove_recovery_account(
        &mut self,
        payer: Pubkey,
        recovery_account: Pubkey,
    ) -> Result<()> {
        if payer != self.owner {
            return Err(errors::ErrorCode::Unauthorized.into());
        }

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

        // reset recovery accounts
        self.recovery_accounts = vec![];

        self.owner = new_owner;
        Ok(())
    }
}

impl Asset {
    pub fn create(
        &mut self,
        authority: Pubkey,
        owner: Pubkey,
        bump: u8,
        metadata_uri: String,
    ) -> Result<()> {
        self.authority = authority;
        self.owner = owner;
        self.attestor = None;
        self.bump = bump;
        self.metadata_uri = metadata_uri;
        Ok(())
    }

    pub fn transfer(
        &mut self,
        payer: Pubkey,
        asset_owner: Pubkey,
        recipient: Pubkey,
    ) -> Result<()> {
        if payer != asset_owner {
            return Err(errors::ErrorCode::Unauthorized.into());
        }

        self.owner = recipient;
        Ok(())
    }

    pub fn attest(&mut self, new_attestor: Pubkey) -> Result<()> {
        self.attestor = Some(new_attestor);
        Ok(())
    }

    pub fn burn(&mut self, amount: u64) -> Result<()> {
        self.owner = Pubkey::default();
        Ok(())
    }
}
