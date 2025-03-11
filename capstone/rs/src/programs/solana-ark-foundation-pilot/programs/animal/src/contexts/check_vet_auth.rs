use anchor_lang::prelude::*;
//use crate::entities::VetAuthority;
use crate::entities::VetAuthority;

#[derive(Accounts)]
pub struct CheckVetAuthority<'info> {
    pub vet: Signer<'info>, // ✅ Vet who is checking their access

    #[account(
        seeds = [b"vet_authority", vet.key().as_ref(), animal.key().as_ref()], // ✅ Check vet-animal relationship
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // ✅ If this exists, vet is authorized
    
    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub animal: AccountInfo<'info>, // ✅ The animal the vet is checking access for
}