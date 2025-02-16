use anchor_lang::prelude::*;
use crate::contexts::animal::*;
use crate::errors::ErrorCode;
 /// Add a new Animal
 pub fn add_animal(
    ctx: Context<AddAnimal>,
    info: [u8; 32],
) -> Result<()> {
    //let payer = &ctx.accounts.payer;
    let owner = &mut ctx.accounts.owner;
    let animal = &mut ctx.accounts.animal;


    // Validate the provided info input
    if info.iter().all(|&byte| byte == 0) {
        return err!(ErrorCode::InvalidMetadata); // Ensure info is not empty
    }

    // Initialize the animal PDA
    animal.info = info;
    animal.owner_id = owner.id.clone();
    animal.id = ctx.accounts.payer.key();

    // Log success
    msg!("Animal added successfully: {}", animal.key());

    Ok(())
}

pub fn update_animal(ctx: Context<UpdateAnimal>, new_info: [u8; 32]) -> Result<()> {
    let animal = &mut ctx.accounts.animal;

    let owner = &mut ctx.accounts.owner;


    // Validate the provided info input
    if new_info.iter().all(|&byte| byte == 0) {
        return err!(ErrorCode::InvalidMetadata); // Ensure info is not empty
    }

    msg!("Updating Animal: {:?}", animal.key());

    // Update the animal's information
    animal.info = new_info;
    animal.owner_id = owner.id.clone();
    animal.id = ctx.accounts.payer.key();

    Ok(())
}

pub fn remove_animal(ctx: Context<RemoveAnimal>) -> Result<()> {
    msg!("Removing Animal: {:?}", ctx.accounts.animal.id);
    // Anchor automatically closes the PDA due to `close = payer` in the context
    Ok(())
}

