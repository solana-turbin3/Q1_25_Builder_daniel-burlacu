use anchor_lang::prelude::*;
use crate::entities::VetAuthority;
use crate::entities::Owner;

#[derive(Accounts)]
pub struct RequestAuthority<'info> {
    #[account(mut)]
    pub requester: Signer<'info>, // The vet/entity requesting authority

    #[account(
        mut,
        seeds = [b"vet_authority", owner.key().as_ref()],
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // Authority PDA

    #[account(mut)]
    pub owner: Account<'info, Owner>, // The animal's owner (for reference)

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveAuthority<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The owner must approve

    #[account(
        mut,
        seeds = [b"vet_authority", owner.key().as_ref()],
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // Authority PDA
}

#[derive(Accounts)]
pub struct RemoveAuthority<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // Only the owner can remove authority

    #[account(
        mut,
        seeds = [b"vet_authority", owner.key().as_ref()],
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // The vet authority PDA
}