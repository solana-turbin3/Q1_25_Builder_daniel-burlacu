// contexts/initialize.rs
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::entities::{Animal, BehaviourRecord}; // Import VetAuthority

#[derive(Accounts)]
pub struct AddBehaviourRecord<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, // Vet adding the record

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"behaviour_record", animal.key().as_ref(), signer.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 8 + 200 // ✅ Adjust space (header + fixed fields + extra for Vec<u8>)
    )]
    pub behaviour_record: Account<'info, BehaviourRecord>, // ✅ The medical record PDA

    #[account(mut)]
    pub animal: Account<'info, Animal>, // ✅ The animal to link the record to

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
