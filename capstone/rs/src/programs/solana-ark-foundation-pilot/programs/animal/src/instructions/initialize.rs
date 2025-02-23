use anchor_lang::prelude::*;
use  crate::contexts::initialize::*;

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

