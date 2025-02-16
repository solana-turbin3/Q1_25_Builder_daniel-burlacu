// contexts/initialize.rs
use anchor_lang::prelude::*;
use crate::entities::veterinary::VeterinaryCabinet;

#[derive(Accounts)]
pub struct Initialize<'info> {
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
    pub system_program: Program<'info, System>, // System program for transferring lamports
}

#[derive(Accounts)]
pub struct UpdateVeterinaryCabinet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    #[account(
        mut,
        seeds =[b"cabinet".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
    )]
    pub cabinet: Account<'info, VeterinaryCabinet>
}