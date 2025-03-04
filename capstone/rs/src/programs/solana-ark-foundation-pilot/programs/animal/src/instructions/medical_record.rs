use std::ops::DerefMut;

use anchor_lang::prelude::*;
//use anchor_lang::solana_program::borsh::try_from_slice_unchecked;
use crate::contexts::medical_record::AddMedicalRecord;
//use crate::entities::VetAuthority;
//use crate::errors::ErrorCode;


// pub fn add_medical_record(
//     ctx: Context<AddMedicalRecord>,
//     record: Vec<u8>,
// ) -> Result<()> {
//     let vet_authority_info = &ctx.accounts.vet_authority;
//     let signer_key = ctx.accounts.signer.key();

//     msg!("🔍 Vet Authority PDA: {:?}", vet_authority_info.key);
//     msg!("🔍 Signer (Vet Wallet) Pubkey: {:?}", signer_key);

//     if vet_authority_info.data_is_empty() {
//         msg!("❌ VetAuthority account does not exist!");
//         return Err(ErrorCode::InvalidVetAuthority.into());
//     }

//     // ✅ Borrow account data safely
//     let vet_authority_data = match vet_authority_info.try_borrow_data() {
//         Ok(data) => data,
//         Err(_) => {
//             msg!("❌ Failed to borrow VetAuthority data");
//             return Err(ErrorCode::InvalidVetAuthority.into());
//         }
//     };

//     // ✅ Deserialize manually to extract VetAuthority
//     let vet_authority: VetAuthority = match try_from_slice_unchecked::<VetAuthority>(&vet_authority_data) {
//         Ok(vet) => vet,
//         Err(err) => {
//             msg!("❌ Failed to deserialize VetAuthority.");
//             msg!("🔍 Raw VetAuthority Data: {:?}", vet_authority_data);
//             msg!("🔍 Expected Size: {}", std::mem::size_of::<VetAuthority>());
//             msg!("🔍 Data Length: {}", vet_authority_data.len());
//             msg!("🔍 Error: {:?}", err);
//             return Err(ErrorCode::InvalidVetAuthority.into());
//         }
//     };

//     // ✅ Log stored vet_pubkey and is_authorized value
//     msg!("✅ Vet Authority Check:");
//     msg!("    🔹 Stored Vet Pubkey: {:?}", vet_authority.vet_pubkey);
//     msg!("    🔹 Signer (Vet Wallet): {:?}", signer_key);
//     msg!("    🔹 is_authorized: {:?}", vet_authority.is_authorized);

//     // ✅ Ensure the vet is authorized and vet_pubkey matches signer
//     require!(
//         vet_authority.is_authorized == 1,
//         ErrorCode::UnauthorizedAccess
//     );

//     require!(
//         vet_authority.vet_pubkey == signer_key,
//         ErrorCode::UnauthorizedAccess
//     );

//     msg!("✅ Vet is authorized and matches stored vet_pubkey!");

//     // ✅ Store medical record data
//     let medical_record = &mut ctx.accounts.medical_record;
//     medical_record.animal_id = *ctx.accounts.animal.key;
//     medical_record.vet = signer_key;
//     medical_record.record = record;
//     medical_record.date = Clock::get()?.unix_timestamp;

//     msg!("✅ Medical record successfully added for Animal: {:?}", medical_record.animal_id);
//     Ok(())
// }

pub fn add_medical_record(
    ctx: Context<AddMedicalRecord>,
    record: Vec<u8>,
) -> Result<()> {
    let signer_key = ctx.accounts.signer.key();

    msg!("🔍 Signer (Vet Wallet) Pubkey: {:?}", signer_key);

    // ✅ Store medical record data
    let medical_record = &mut ctx.accounts.medical_record;
    medical_record.animal_id = ctx.accounts.animal.key();
    medical_record.vet = signer_key;
    medical_record.record = record;
    medical_record.date = Clock::get()?.unix_timestamp;

    msg!("✅ Medical record successfully added for Animal: {:?}", medical_record.animal_id);

    // ✅ Explicitly Serialize Updated Data
    let medical_record_info = medical_record.to_account_info();
    let mut medical_record_data = medical_record_info.data.borrow_mut();
    medical_record.try_serialize(&mut medical_record_data.deref_mut())?;

    Ok(())
}




