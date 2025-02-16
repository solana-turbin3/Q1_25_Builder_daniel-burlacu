use anchor_lang::prelude::*;

#[account]
pub struct VetAuthority {
    pub is_authorized: bool,
    pub vet_pubkey: Pubkey,
}