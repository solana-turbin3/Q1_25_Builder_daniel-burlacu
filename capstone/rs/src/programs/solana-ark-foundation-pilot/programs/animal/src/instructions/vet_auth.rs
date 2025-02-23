use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::contexts::vet_auth::{RemoveAuthority, ApproveAuthority, RequestAuthority}; // ✅ Fix: Explicitly import missing structs

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

pub fn remove_authority(ctx: Context<RemoveAuthority>) -> Result<()> {
    let vet_authority = &mut ctx.accounts.vet_authority;

    // Ensure the authority exists before removing it
    require!(
        vet_authority.is_authorized,
        ErrorCode::UnauthorizedAccess
    );

    vet_authority.is_authorized = false; // Revoke authorization

    msg!("Vet Authority access revoked by Owner {:?}", ctx.accounts.owner.key());
    Ok(())
}
