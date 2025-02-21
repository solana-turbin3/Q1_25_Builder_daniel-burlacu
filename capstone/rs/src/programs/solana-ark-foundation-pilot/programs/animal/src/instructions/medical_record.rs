use anchor_lang::prelude::*;
use crate::contexts::medical_record::AddMedicalRecord;
use crate::entities::VetAuthority;
use crate::errors::ErrorCode;


pub fn add_medical_record(
    ctx: Context<AddMedicalRecord>,
    record: Vec<u8>,
) -> Result<()> {
    let _authority_check: VetAuthority = VetAuthority {
        is_authorized: false,
        vet_pubkey: Pubkey::default(),
    };// Ensure only authorized entities can add medical records
    
    require!(
        vet_authority.is_authorized && vet_authority.vet_pubkey == ctx.accounts.signer.key(),
        ErrorCode::UnauthorizedAccess
    );

    let medical_record = &mut ctx.accounts.medical_record; // Fix variable name
    medical_record.record = record; // Now modifying the correct struct
    msg!("Medical record added");
    Ok(())
}
