/// Add a new Animal Owner
use anchor_lang::prelude::*;
use crate::contexts::owner::*;
use crate::errors::ErrorCode;

pub fn add_owner(
    ctx: Context<AddOwner>,
    info: [u8; 32],
) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let cabinet = &mut ctx.accounts.cabinet;
    //let payer = &ctx.accounts.payer;
   
    // Validate the provided info input
    if info.iter().all(|&byte| byte == 0) {
        return err!(ErrorCode::InvalidMetadata); // Ensure info is not empty
    }
    
    // Initialize the owner PDA
    owner.info = info;
    owner.veterinary_cabinet_id = cabinet.key();
    owner.id = ctx.accounts.payer.key();
    

     // Log success
     msg!("Animal owner added successfully: {}", cabinet.key());

    Ok(())
}

pub fn update_animal_owner(ctx: Context<UpdateOwner>, new_info: [u8; 32]) -> Result<()> {
    let owner = &mut ctx.accounts.owner;

    msg!("Updating Animal Owner: {:?}", owner.key());

    // Update the owner's information
    owner.info = new_info;

    Ok(())
}

pub fn remove_animal_owner(ctx: Context<RemoveOwner>) -> Result<()> {
    msg!("Removing Animal Owner: {:?}", ctx.accounts.owner.id);
    // Anchor automatically closes the PDA due to `close = payer` in the context
    Ok(())
}