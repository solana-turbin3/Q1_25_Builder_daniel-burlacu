use anchor_lang::prelude::*;
use crate::contexts::behaviour_record::AddBehaviourRecord;
use std::ops::DerefMut;

pub fn add_behaviour_record(
    ctx: Context<AddBehaviourRecord>,
    record: Vec<u8>,
) -> Result<()> {
    let signer_key = ctx.accounts.signer.key();

    msg!("🔍 Signer (Vet Wallet) Pubkey: {:?}", signer_key);

    // ✅ Store medical record data
    let behaviour_record = &mut ctx.accounts.behaviour_record;
    behaviour_record.animal_id = ctx.accounts.animal.key();
    behaviour_record.vet = signer_key;
    behaviour_record.record = record;
    behaviour_record.date = Clock::get()?.unix_timestamp;
    
    msg!("✅ Medical record successfully added for Animal: {:?}", behaviour_record.animal_id);

     // ✅ Explicitly Serialize Updated Data
     let behaviour_record_info =behaviour_record.to_account_info();
     let mut behaviour_record_data = behaviour_record_info.data.borrow_mut();
     behaviour_record.try_serialize(&mut behaviour_record_data.deref_mut())?;
 
     Ok(())
}
