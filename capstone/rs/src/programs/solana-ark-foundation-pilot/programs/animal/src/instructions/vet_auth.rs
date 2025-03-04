use std::ops::DerefMut;

use anchor_lang::prelude::*;
use crate::contexts::vet_auth:: RequestAuthority; // ✅ Fix: Explicitly import missing structs
use crate::contexts::{CheckPendingRequests, CheckVetAuthority};
use crate::entities::VetAuthority;
use crate::{entities::AuthorityRequest,errors::ErrorCode};
use crate::ApproveOrRejectAuthority;

pub fn request_authority(ctx: Context<RequestAuthority>) -> Result<()> {
    let authority_request = &mut ctx.accounts.authority_request;

    authority_request.vet_pubkey = ctx.accounts.requester.key();
    authority_request.owner_pubkey = ctx.accounts.owner.key();
    authority_request.animal_pubkey = ctx.accounts.animal.key();
    authority_request.status = 0; // Set status to "Pending"

    msg!("✅ Authority request created by {:?}", authority_request.vet_pubkey);
    Ok(())
}

pub fn approve_or_reject_authority(ctx: Context<ApproveOrRejectAuthority>, decision: u8) -> Result<()> {
    // ✅ Manually borrow `authority_request` and deserialize it
    let mut authority_request = AuthorityRequest::try_deserialize(&mut ctx.accounts.authority_request.data.borrow_mut().as_ref())?;

    // ✅ Ensure the request is still pending
    require!(authority_request.status == 0, ErrorCode::InvalidRequestStatus);

    // ✅ Compute expected PDA
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            b"vet_authority",
            ctx.accounts.veterinary.key().as_ref(),
            ctx.accounts.animal.key().as_ref(),
        ],
        ctx.program_id,
    );

    msg!("🔍 Expected VetAuthority PDA: {:?}", expected_pda);

    // ✅ Ensure VetAuthority PDA matches
    require_keys_eq!(
        ctx.accounts.vet_authority.key(),
        expected_pda,
        ErrorCode::InvalidVetAuthority
    );

    // ✅ Initialize VetAuthority if it doesn't exist
    let vet_authority = &mut ctx.accounts.vet_authority;
    if vet_authority.vet_pubkey == Pubkey::default() {
        msg!("🔍 Initializing new VetAuthority...");
        vet_authority.vet_pubkey = ctx.accounts.veterinary.key();
        vet_authority.animal_pubkey = ctx.accounts.animal.key();
        vet_authority.is_authorized = 0; // Default: Not authorized
    }

    // ✅ Approve or Reject the request
    if decision == 1 {
        msg!("✅ Authority approved for {:?}", ctx.accounts.veterinary.key());
        vet_authority.is_authorized = 1; // ✅ Set to authorized
        authority_request.status = 1; // ✅ Mark request as approved
    } else {
        msg!("❌ Authority request denied.");
        vet_authority.is_authorized = 0; // ✅ Set explicitly to `0`
        authority_request.status = 2; // ✅ Mark request as rejected
    }

    // ✅ Serialize modified authority request back
    authority_request.try_serialize(&mut ctx.accounts.authority_request.data.borrow_mut().deref_mut())?;

    msg!(
        "✅ Request updated. Vet: {:?}, Status: {}",
        authority_request.vet_pubkey,
        authority_request.status
    );

    Ok(())
}



pub fn check_vet_authority(ctx: Context<CheckVetAuthority>) -> Result<()> {
    let vet_authority = &ctx.accounts.vet_authority;

    if vet_authority.is_authorized == 1 { // ✅ Now explicitly checking against `1`
        msg!(
            "✅ Vet {:?} is authorized for animal {:?}",
            vet_authority.vet_pubkey,
            vet_authority.animal_pubkey
        );
    } else {
        msg!(
            "❌ Vet {:?} is NOT authorized for animal {:?}",
            vet_authority.vet_pubkey,
            vet_authority.animal_pubkey
        );
    }
    
    Ok(())
}

pub fn check_pending_requests<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CheckPendingRequests<'info>>,
) -> Result<()>
where
    'c: 'info,
{
    msg!("🔍 Scanning for pending requests...");

    for account_info in ctx.remaining_accounts.iter() {
        let authority_request = Account::<AuthorityRequest>::try_from(account_info);

        match authority_request {
            Ok(request) if request.status == 0 => {
                msg!("✅ Pending Request Found: {:?}", request.key());
            }
            Err(_) => {
                msg!("❌ Skipping invalid account {:?}", account_info.key);
            }
            _ => {} // Ignore non-pending requests
        }
    }

    Ok(())
}

// pub fn approve_request(ctx: Context<ApproveAuthority>) -> Result<()> {
//     let vet_authority = &mut ctx.accounts.vet_authority;
//     vet_authority.is_authorized = true;

//     msg!("Approved authority for {:?}", vet_authority.vet_pubkey);
//     Ok(())
// }

// pub fn remove_authority(ctx: Context<RemoveAuthority>) -> Result<()> {
//     let vet_authority = &mut ctx.accounts.vet_authority;

//     // Ensure the authority exists before removing it
//     require!(
//         vet_authority.is_authorized,
//         ErrorCode::UnauthorizedAccess
//     );

//     vet_authority.is_authorized = false; // Revoke authorization

//     msg!("Vet Authority access revoked by Owner {:?}", ctx.accounts.owner.key());
//     Ok(())
// }
