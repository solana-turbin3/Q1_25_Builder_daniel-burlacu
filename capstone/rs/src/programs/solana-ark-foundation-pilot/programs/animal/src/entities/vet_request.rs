#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

#[account]
pub struct AuthorityRequest {
    pub vet_pubkey: Pubkey,    // Vet cabinet requesting authority
    pub owner_pubkey: Pubkey,  // Animal owner
    pub animal_pubkey: Pubkey, // The animal the vet is requesting access to
    pub status: u8,            // 0 = Pending, 1 = Approved, 2 = Rejected
}

#[derive(Accounts)]
pub struct CheckPendingRequests<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The owner checking requests
}