use anchor_lang::prelude::*;
use crate::entities::TreasuryAccount;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {

    pub admin: Signer<'info>, // Admin as a separate signer

    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"treasury".as_ref()], // Unique seed tied to admin
        bump,
        space = 8 + 256// Account discriminator + TreasuryAccount fields
    )]
    pub treasury: Account<'info, TreasuryAccount>,

    #[account(
        seeds = [b"admin".as_ref()], // Seed for Admin PDA
        bump,
        has_one = admin // Ensure the admin matches the Admin PDA
    )]
    pub admin_pda: Account<'info, AdminAccount>,

    pub system_program: Program<'info, System>, // System program for lamport transfers
}

#[derive(Accounts)]
pub struct WithdrawFromTreasury<'info> {
    #[account(
        mut,
        seeds = [b"treasury".as_ref(), admin.key().as_ref()], // Consistent seed with InitializeTreasury
        bump,
        has_one = admin // Ensure only the admin can withdraw
    )]
    pub treasury: Account<'info, TreasuryAccount>,

    #[account(
        seeds = [b"admin".as_ref()], // Seed for Admin PDA
        bump
    )]
    pub admin_pda: Account<'info, AdminAccount>,

    #[account(mut)]
    pub admin: Signer<'info>, // Admin wallet signs the transaction

    /// CHECK: Destination account for funds
    #[account(mut)]
    pub destination: AccountInfo<'info>, // Optional: add constraints if needed

    pub system_program: Program<'info, System>, // System program for lamport transfers
}