use anchor_lang::prelude::*;

#[account]
pub struct AdminAccount {
    pub key: Pubkey,   // Admin PDA public key
    pub admin: Pubkey, // Admin wallet public key
}
