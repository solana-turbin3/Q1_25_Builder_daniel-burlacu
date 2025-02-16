use anchor_lang::prelude::*;
use crate::contexts::medical_record::AddMedicalRecord;


pub fn add_medical_record(
    ctx: Context<AddMedicalRecord>,
    record: Vec<u8>,
) -> Result<()> {
    msg!("Adding new behaviour data record PDA");
    let medical_record = &mut ctx.accounts.medical_record; // Fix variable name
    medical_record.record = record; // Now modifying the correct struct
    Ok(())
}
