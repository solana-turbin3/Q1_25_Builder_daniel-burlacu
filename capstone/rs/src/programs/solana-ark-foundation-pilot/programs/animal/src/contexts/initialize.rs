// contexts/initialize.rs
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::entities::animal::Animal;
use crate::entities::owner::Owner;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation
    
    #[account(
        init_if_needed,
        payer = payer,
        seeds =[b"animal".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
        space = 8 + 256 // Account discriminator + VeterinaryCabinet fields
    )]
    pub animal: Account<'info, Animal>,

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
pub struct UpdateAnimal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    #[account(
        mut,
        seeds =[b"animal".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
    )]
    pub animal: Account<'info, Animal>
}

#[derive(Accounts)]
pub struct ApproveVet<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The owner must sign to approve

    pub system_program: Program<'info, System>,
}
