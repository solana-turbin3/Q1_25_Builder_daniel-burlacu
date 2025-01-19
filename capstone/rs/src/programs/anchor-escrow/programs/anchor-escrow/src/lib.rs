pub mod context;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("4W2xEwJ7K4Detthv2Qc6fZRV9rjM7RxfnKXVR5okaP9S");

pub use context::*;
pub use state::*;

#[program]
pub mod anchor_escrow {
    use super::*;
    
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}
