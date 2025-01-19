use anchor_lang::prelude::*;
use crate::contexts::animal::AddAnimal;
use crate::errors::ErrorCode;

 /// Add a new Animal
 pub fn add_animal(
    ctx: Context<AddAnimal>,
    info: [u8; 32],
) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let cabinet = &mut ctx.accounts.cabinet;
    let owner = &mut ctx.accounts.owner;
    let animal = &mut ctx.accounts.animal;

    // Ensure the caller is a recognized veterinary cabinet
    if cabinet.id != payer.key() {
        return err!(ErrorCode::UnauthorizedAccess);
    }

    // Validate the provided info input
    if info.iter().all(|&byte| byte == 0) {
        return err!(ErrorCode::InvalidMetadata); // Ensure info is not empty
    }

    // Initialize the animal PDA
    animal.info = info;
    animal.owner_id = owner.id.clone();
    animal.veterinary_cabinet_id = cabinet.id.clone();
    animal.id = ctx.accounts.payer.key();

    // Log success
    msg!("Animal added successfully: {}", animal.key());

    Ok(())
}


// pub fn transfer_animal_ownership(
//     ctx: Context<TransferOwnership>,
//     new_owner_id: String,
//     veterinary_cabinet_id: String,
// ) -> Result<()> {
//     let animal = &mut ctx.accounts.animal;

//     // Ensure the caller is a recognized veterinary cabinet
//     let cabinet_id = veterinary_cabinet_id.clone(); // Fix shadowing and cloning
//     if cabinet_id != animal.cabinet_id {
//         return err!(ErrorCode::UnauthorizedAccess);
//     }

//     // Update the owner ID for the animal PDA
//     animal.owner_id = new_owner_id;

//     Ok(())
// }
