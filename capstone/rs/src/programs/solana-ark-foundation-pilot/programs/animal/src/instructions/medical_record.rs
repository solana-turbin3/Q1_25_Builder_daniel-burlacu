use anchor_lang::prelude::*;
use crate::contexts::medical_record::AddMedicalRecord;
use crate::errors::ErrorCode;


pub fn add_medical_record(
    ctx: Context<AddMedicalRecord>,
    record: Vec<u8>,
) -> Result<()> {
    let vet_authority = &ctx.accounts.vet_authority;
    
    require!(
        vet_authority.is_authorized && vet_authority.vet_pubkey == ctx.accounts.signer.key(),
        ErrorCode::UnauthorizedAccess
    );

    let medical_record = &mut ctx.accounts.medical_record; // Fix variable name
    medical_record.record = record; // Now modifying the correct struct
    msg!("Medical record added");
    Ok(())
}
