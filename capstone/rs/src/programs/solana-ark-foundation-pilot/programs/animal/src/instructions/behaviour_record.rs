use anchor_lang::prelude::*;
use crate::contexts::behaviour_record::AddBehaviourRecord;
use crate::entities::VetAuthority;
use crate::errors::ErrorCode;
use std::ops::DerefMut;

pub fn add_behaviour_record(
    ctx: Context<AddBehaviourRecord>,
    record: Vec<u8>,
) -> Result<()> {
    let signer_key = ctx.accounts.signer.key();
    let behaviour_record = &mut ctx.accounts.behaviour_record;

    // ✅ Check if the medical record is already initialized
    if behaviour_record.animal_id != Pubkey::default() {
        msg!("❌ Medical record already exists!");
    }else {

    msg!("🔍 Signer (Vet Wallet) Pubkey: {:?}", signer_key);

    // ✅ Compute Expected VetAuthority PDA
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            b"vet_authority",
            signer_key.as_ref(),  // Vet's Public Key
            ctx.accounts.animal.key().as_ref(), // Animal's Public Key
        ],
        ctx.program_id,
    );

    msg!("✅ Expected VetAuthority PDA: {:?}", expected_pda);
    msg!("✅ Passed VetAuthority PDA: {:?}", ctx.accounts.vet_authority.key());

    // ✅ Ensure VetAuthority PDA matches the expected PDA
    require_keys_eq!(
        ctx.accounts.vet_authority.key(),
        expected_pda,
        ErrorCode::InvalidVetAuthority
    );

    // ✅ Ensure vet_authority is owned by expected program
    if ctx.accounts.vet_authority.owner != ctx.program_id {
        msg!("❌ VetAuthority account is not owned by expected program!");
        return Err(ErrorCode::InvalidVetAuthority.into());
    }
    msg!("✅ VetAuthority is owned by the expected program.");

   // ✅ Borrow account data safely
    let vet_authority_data = match ctx.accounts.vet_authority.try_borrow_mut_data() {
      Ok(data) => data,
      Err(_) => {
        msg!("❌ Failed to borrow VetAuthority data");
        return Err(ErrorCode::InvalidVetAuthority.into());
      }
    };

    // ✅ Deserialize manually to extract VetAuthority
    let vet_authority = match VetAuthority::try_deserialize(&mut vet_authority_data.as_ref()) {
        Ok(vet) => vet,
        Err(err) => {
            msg!("❌ Failed to deserialize VetAuthority.");
            msg!("🔍 Raw VetAuthority Data: {:?}", vet_authority_data);
            msg!("🔍 Expected Size: {}", std::mem::size_of::<VetAuthority>());
            msg!("🔍 Data Length: {}", vet_authority_data.len());
            msg!("🔍 Error: {:?}", err);
            return Err(ErrorCode::InvalidVetAuthority.into());
        }
    };

    // ✅ Logging VetAuthority Data
    msg!("✅ VetAuthority PDA: {:?}", ctx.accounts.vet_authority.key());
    msg!("✅ VetAuthority Owner: {:?}", ctx.accounts.vet_authority.owner);
    msg!("✅ Stored Vet Pubkey: {:?}", vet_authority.vet_pubkey);
    msg!("✅ Stored Animal Pubkey: {:?}", vet_authority.animal_pubkey);
    msg!("✅ VetAuthority is_authorized: {:?}", vet_authority.is_authorized);
    msg!("✅ Expected Program ID: {:?}", ctx.program_id);

    // ✅ Ensure the vet is authorized and vet_pubkey matches signer
    require!(
        vet_authority.is_authorized == 1, // ✅ Only approved vets can add records
        ErrorCode::UnauthorizedAccess
    );

    require_keys_eq!(
        vet_authority.vet_pubkey,
        signer_key,
        ErrorCode::UnauthorizedAccess
    );

    msg!("✅ Vet is authorized and matches stored vet_pubkey!");
    
    const MAX_RECORD_SIZE: usize = 500; // ✅ Set a reasonable limit

    if  record.len() > MAX_RECORD_SIZE {
        msg!("❌ Medical record is too large!");
        return Err(ErrorCode::RecordTooLarge.into());
     }
 
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
    }
     Ok(())
}
