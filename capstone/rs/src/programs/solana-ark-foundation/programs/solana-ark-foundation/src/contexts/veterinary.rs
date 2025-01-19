use anchor_lang::prelude::*;
use crate::entities::VeterinaryCabinet;
use crate::entities::TreasuryAccount;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct AddVeterinaryCabinet<'info> {

    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    #[account(
        init_if_needed,
        payer = payer,
        seeds =[b"cabinet".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
        space = 8 + 256 // Account discriminator + VeterinaryCabinet fields
    )]
    pub cabinet: Account<'info, VeterinaryCabinet>,

    #[account(
        init_if_needed,
        payer = payer, // Payer funds the initialization
        seeds = [b"treasury".as_ref()], // Seed for Treasury PDA based on admin key
        bump,
        space = 8 + 256 // Account discriminator + TreasuryAccount fields
    )]
    pub treasury: Account<'info, TreasuryAccount>, // Treasury PDA to receive the JOIN_FEE

    #[account(
        seeds = [b"admin".as_ref()], // Seed for Admin PDA
        bump
    )]
    pub admin_pda: Account<'info, AdminAccount>, // Admin PDA for verification

    #[account(mut)]
    pub admin: Signer<'info>, // Admin wallet signs the transaction

    pub system_program: Program<'info, System>, // System program for transferring lamports
}