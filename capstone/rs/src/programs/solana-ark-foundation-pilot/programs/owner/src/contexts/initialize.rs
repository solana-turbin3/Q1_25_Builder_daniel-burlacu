// contexts/initialize.rs
use anchor_lang::prelude::*;
use crate::entities::owner::Owner;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    #[account(
        init_if_needed,
        payer = payer,
        seeds =[b"owner".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
        space = 8 + 256 // Account discriminator + VeterinaryCabinet fields
    )]
    pub owner: Account<'info, Owner>,
    pub system_program: Program<'info, System>, // System program for transferring lamports
}

#[derive(Accounts)]
pub struct UpdateOwner<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    #[account(
        mut,
        seeds =[b"owner".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
    )]
    pub owner: Account<'info, Owner>
}