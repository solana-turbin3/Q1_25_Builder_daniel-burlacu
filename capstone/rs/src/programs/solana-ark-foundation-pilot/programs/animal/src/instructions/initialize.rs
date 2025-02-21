use anchor_lang::prelude::*;
use  crate::contexts::initialize::*;
use crate::errors::ErrorCode;

pub fn initialize(
    ctx: Context<Initialize>, 
    info: [u8; 32], // Fixed-length info for the veterinary cabinet
) -> Result<()> {
    msg!("Initializing Animal PDA");
    let animal = &mut ctx.accounts.animal;
    let owner = &mut ctx.accounts.owner;

    // Set animal info
    animal.info = info;
    animal.owner_id = owner.owner_id; // Link animal to the owner
    
    msg!("Animal Created: Owner {:?}", animal.owner_id);
    Ok(())
}

pub fn update_animal(
    ctx: Context<UpdateAnimal>, 
    new_info: [u8; 32], 
) -> Result<()> {
    let vet_authority = &ctx.accounts.vet_authority;
    require!(
        vet_authority.is_authorized && vet_authority.vet_pubkey == ctx.accounts.payer.key(),
        ErrorCode::UnauthorizedAccess
    );

    let animal = &mut ctx.accounts.animal;
    animal.info = new_info;

    msg!("Animal information updated");
    Ok(())
}
