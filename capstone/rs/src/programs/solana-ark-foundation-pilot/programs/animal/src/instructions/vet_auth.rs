use anchor_lang::prelude::*;
use crate::entities::VetAuthority;
use crate::{entities::AuthorityRequest,errors::ErrorCode};
use crate::ApproveOrRejectAuthority;



pub fn approve_or_reject_authority(ctx: Context<ApproveOrRejectAuthority>, decision: u8) -> Result<()> {
   
   // ✅ Manually borrow and deserialize `authority_request`
   let authority_request_data = &mut ctx.accounts.authority_request.try_borrow_mut_data()?;
   let mut authority_request = AuthorityRequest::try_deserialize(&mut authority_request_data.as_ref())?;

    // ✅ Ensure the request is still pending
    require!(authority_request.status == 0, ErrorCode::InvalidRequestStatus);

    // ✅ Extract veterinary public key

    let vet_authority = &mut ctx.accounts.vet_authority;

if vet_authority.vet_pubkey == Pubkey::default() {
    msg!("🛠️ VetAuthority does not exist, initializing...");

    vet_authority.vet_pubkey = ctx.accounts.veterinary.key();
    vet_authority.animal_pubkey = ctx.accounts.animal.key();
    vet_authority.is_authorized = decision;
    authority_request.status = decision;

    msg!("✅ New VetAuthority created!");
} else {
    msg!("🔄 VetAuthority exists, updating status...");
    vet_authority.is_authorized = decision;
    authority_request.status = decision;
}
    msg!("🔍 Expected VetAuthority Size: {}", std::mem::size_of::<VetAuthority>());  

    msg!("✅ Passed Veterinary Pubkey: {:?}", ctx.accounts.veterinary.key());
    msg!("✅ Stored Vet Pubkey: {:?}", vet_authority.vet_pubkey);
    msg!("✅ Passed Animal Pubkey: {:?}", ctx.accounts.animal.key());
    msg!("✅ Stored Animal Pubkey: {:?}", vet_authority.animal_pubkey);
    
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