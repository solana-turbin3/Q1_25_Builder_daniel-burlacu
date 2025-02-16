use anchor_lang::prelude::*;
use  crate::contexts::initialize::*;

pub fn initialize(
    ctx: Context<Initialize>, 
    info: [u8; 32], // Fixed-length info for the veterinary cabinet
) -> Result<()> {
    msg!("Initializing Veterinary PDA");
    let cabinet = &mut ctx.accounts.cabinet; // Fix variable name
    cabinet.info = info; // Now modifying the correct struct
    Ok(())
}

pub fn update_veterinary_abinet(
    ctx: Context<UpdateVeterinaryCabinet>, 
    new_info: [u8; 32], // Fixed-length info for the veterinary cabinet
) -> Result<()> {
    msg!("Updating Veterinary Cabinet");
    let cabinet = &mut ctx.accounts.cabinet; // Fix variable name
    cabinet.info = new_info; // Now modifying the correct struct
    Ok(())
}