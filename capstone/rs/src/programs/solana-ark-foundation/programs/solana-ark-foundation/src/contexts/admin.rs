use anchor_lang::prelude::*;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 64, // Adjust for AdminAccount size
        seeds = [b"admin".as_ref()],
        bump
    )]
    pub admin_pda: Account<'info, AdminAccount>,

    #[account(mut)]
    pub payer: Signer<'info>, // Wallet paying for the transaction
    pub admin: Signer<'info>, // Admin as a separate signer
    pub system_program: Program<'info, System>,
}