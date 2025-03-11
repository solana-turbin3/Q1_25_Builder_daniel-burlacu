use anchor_lang::prelude::*;
use crate::contexts::req_auth:: RequestAuthority; // ✅ Fix: Explicitly import missing structs


pub fn request_authority(ctx: Context<RequestAuthority>) -> Result<()> {
    let authority_request = &mut ctx.accounts.authority_request;

    authority_request.vet_pubkey = ctx.accounts.requester.key();
    authority_request.owner_pubkey = ctx.accounts.owner.key();
    authority_request.animal_pubkey = ctx.accounts.animal.key();
    authority_request.status = 0; // Set status to "Pending"

    msg!("✅ Authority request created by {:?}", authority_request.vet_pubkey);
    Ok(())
}