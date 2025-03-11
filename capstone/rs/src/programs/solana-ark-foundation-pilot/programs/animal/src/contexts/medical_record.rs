// contexts/initialize.rs
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::entities::{Animal, MedicalRecord}; // Import VetAuthority

// #[derive(Accounts)]
// pub struct AddMedicalRecord<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>, // Vet adding the record

//     #[account(
//         init_if_needed,
//         payer = signer,
//         seeds = [b"medical_record", animal.key().as_ref(), signer.key().as_ref()],
//         bump,
//         space = 8 + 32 + 32 + 8 + 200 // ✅ Adjust space (header + fixed fields + extra for Vec<u8>)
//     )]
//     pub medical_record: Account<'info, MedicalRecord>, // ✅ The medical record PDA

//     #[account(mut)]
//     pub animal: Account<'info, Animal>, // ✅ The animal to link the record to

//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct AddMedicalRecord<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // ✅ Vet signing the transaction

    /// CHECK: Vet Authority PDA comes from another program, needs manual verification
     #[account(mut)]
     pub vet_authority: AccountInfo<'info>, // ✅ Vet authority from external program

    #[account(
        init_if_needed,  // ✅ Will only create if it does not exist
        payer = signer,
        seeds = [b"medical_record", animal.key().as_ref(), signer.key().as_ref()], // ✅ Link record to both vet and animal
        bump,
        space = 8 + std::mem::size_of::<MedicalRecord>() + 512 // ✅ Extra space for `Vec<u8>` growth
    )]
    pub medical_record: Account<'info, MedicalRecord>, // ✅ Medical record PDA

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub animal: AccountInfo<'info>, // ✅ The animal the record belongs to

    pub system_program: Program<'info, System>,
}
