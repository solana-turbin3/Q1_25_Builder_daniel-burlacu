// contexts/initialize.rs
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::entities::BehaviourRecord; // Import VetAuthority

#[derive(Accounts)]
pub struct AddBehaviourRecord<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // ✅ Vet signing the transaction

    /// CHECK: Vet Authority PDA comes from another program, needs manual verification
     #[account(mut)]
     pub vet_authority: AccountInfo<'info>, // ✅ Vet authority from external program

    #[account(
        init_if_needed,  // ✅ Will only create if it does not exist
        payer = signer,
        seeds = [b"behaviour_record", animal.key().as_ref(), signer.key().as_ref()], // ✅ Link record to both vet and animal
        bump,
        space = 8 + std::mem::size_of::<BehaviourRecord>() + 512 // ✅ Extra space for `Vec<u8>` growth
    )]
    pub behaviour_record: Account<'info, BehaviourRecord>, // ✅ Behaviour record PDA

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub animal: AccountInfo<'info>, // ✅ The animal the record belongs to

    pub system_program: Program<'info, System>,
}
// pub struct AddBehaviourRecord<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>,

//     #[account(
//         mut,
//         seeds = [b"vet_authority", owner.key().as_ref()],
//         bump
//     )]
//     pub vet_authority: Account<'info, VetAuthority>, // New VetAuthority field

//     #[account(
//         init,
//         payer = owner,
//         seeds = [b"behaviour_record", owner.key().as_ref()],
//         bump,
//         space = 8 + 512
//     )]
//     pub behaviour_record: Account<'info, BehaviourRecord>, // New medical record

//     pub system_program: Program<'info, System>,
// }
