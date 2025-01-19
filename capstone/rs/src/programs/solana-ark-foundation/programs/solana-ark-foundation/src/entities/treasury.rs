use anchor_lang::prelude::*;

#[account]
pub struct TreasuryAccount {
    pub balance: u64,       // Total balance in the treasury
    pub admin: Pubkey,      // Admin PDA that owns the treasury
}