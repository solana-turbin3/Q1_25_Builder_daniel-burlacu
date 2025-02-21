// contexts/initialize.rs
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::entities::{MedicalRecord, VetAuthority}; // Import VetAuthority

#[derive(Accounts)]
pub struct AddMedicalRecord<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vet_authority", owner.key().as_ref()],
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // New VetAuthority field

     #[account(
        init,
        payer = signer,
        seeds = [b"medical_record", signer.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 8 // Adjusted for MedicalRecord struct
    )]
    pub medical_record: Account<'info, MedicalRecord>, // Medical record PDA

    #[account(mut)]
    pub owner: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
