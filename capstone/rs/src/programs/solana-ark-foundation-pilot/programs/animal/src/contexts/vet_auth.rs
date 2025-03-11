use anchor_lang::prelude::*;
//use crate::entities::VetAuthority;
use crate::entities::VetAuthority;

#[derive(Accounts)]
pub struct ApproveOrRejectAuthority<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The animal owner approving/rejecting

    /// CHECK: Explicitly pass veterinary cabinet account (previously derived from authority_request)
    pub veterinary: AccountInfo<'info>,

    /// CHECK: Explicitly pass animal account (previously derived from authority_request)
    pub animal: AccountInfo<'info>,

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub authority_request: AccountInfo<'info>, // The pending request

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"vet_authority", veterinary.key().as_ref(), animal.key().as_ref()], // ✅ This is how it's being derived
        bump,
        space = 8 + std::mem::size_of::<VetAuthority>()
    )]
    pub vet_authority: Account<'info, VetAuthority>,
    
    pub system_program: Program<'info, System>,
}

