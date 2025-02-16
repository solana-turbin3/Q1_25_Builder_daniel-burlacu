// contexts/initialize.rs
use anchor_lang::prelude::*;
use crate::entities::medical_record::MedicalRecord;

#[derive(Accounts)]
pub struct AddMedicalRecord<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"medical_record", signer.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 8 // Adjusted for MedicalRecord struct
    )]
    pub medical_record: Account<'info, MedicalRecord>, // New medical record

    #[account(mut)]
    pub owner: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
