use anchor_lang::prelude::*;
use  crate::contexts::initialize::*;

pub fn initialize(
    ctx: Context<Initialize>, 
    info: [u8; 32], // Fixed-length info for the veterinary cabinet
) -> Result<()> {
    msg!("Initializing Animal PDA");
    let animal = &mut ctx.accounts.animal; // Fix variable name
    animal.info = info; // Now modifying the correct struct
    Ok(())
}

pub fn update_animal(
    ctx: Context<UpdateAnimal>, 
    new_info: [u8; 32], // Fixed-length info for the veterinary cabinet
) -> Result<()> {
    msg!("Updating Animal");
    let animal = &mut ctx.accounts.animal; // Fix variable name
    animal.info = new_info; // Now modifying the correct struct
    Ok(())
}