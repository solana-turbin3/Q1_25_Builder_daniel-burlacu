use anchor_lang::prelude::*;
use crate::entities::{VetAuthority, Owner};

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

pub fn request_authority(ctx: Context<RequestAuthority>) -> Result<()> {
    let vet_authority = &mut ctx.accounts.vet_authority;
    vet_authority.is_authorized = false;
    vet_authority.vet_pubkey = ctx.accounts.requester.key();

    msg!("Authority request created by {:?}", vet_authority.vet_pubkey);
    Ok(())
}

pub fn approve_request(ctx: Context<ApproveAuthority>) -> Result<()> {
    let vet_authority = &mut ctx.accounts.vet_authority;
    vet_authority.is_authorized = true;

    msg!("Approved authority for {:?}", vet_authority.vet_pubkey);
    Ok(())
}


