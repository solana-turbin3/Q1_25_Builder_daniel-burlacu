use anchor_lang::prelude::*;
use crate::entities::{Animal, VetAuthority};

#[derive(Accounts)]  // ✅ This is required for Anchor to recognize the struct
pub struct UpdateAnimal<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The vet or authorized user

    #[account(
        mut,
        seeds = [b"vet_authority", payer.key().as_ref()],
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // Required for authorization

    #[account(
        mut,
        seeds = [b"animal", payer.key().as_ref()],
        bump
    )]
    pub animal: Account<'info, Animal>,

    pub system_program: Program<'info, System>,
}