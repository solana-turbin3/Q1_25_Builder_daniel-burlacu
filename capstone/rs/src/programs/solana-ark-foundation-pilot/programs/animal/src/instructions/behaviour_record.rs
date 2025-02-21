use anchor_lang::prelude::*;
use crate::contexts::behaviour_record::AddBehaviourRecord;
use crate::entities::VetAuthority;
use crate::errors::ErrorCode;


pub fn add_behaviour_record(
  ctx: Context<AddBehaviourRecord>,
  record: Vec<u8>,
) -> Result<()> {
   
  let _authority_check: VetAuthority = VetAuthority {
    is_authorized: false,
    vet_pubkey: Pubkey::default(),
};// Ensure only authorized entities can add medical records
  
  require!(
      vet_authority.is_authorized && vet_authority.vet_pubkey == ctx.accounts.owner.key(),
      ErrorCode::UnauthorizedAccess
  );

    let behaviour_record = &mut ctx.accounts.behaviour_record; // Fix variable name
    behaviour_record.record = record; // Now modifying the correct struct

    msg!("Adding new behaviour data record PDA");
    Ok(())
}
