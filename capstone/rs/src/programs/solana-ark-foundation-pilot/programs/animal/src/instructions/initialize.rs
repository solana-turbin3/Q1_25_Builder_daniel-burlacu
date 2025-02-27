use anchor_lang::prelude::*;
use  crate::contexts::initialize::*;
use crate::errors::ErrorCode;

pub fn initialize(
    ctx: Context<Initialize>, 
    info: [u8; 32],
) -> Result<()> {
    msg!("Initializing Animal PDA");

    let animal = &mut ctx.accounts.animal;
    let owner = &ctx.accounts.owner; // ✅ Reference external owner

     // ✅ Check that the provided owner is actually owned by the external program
    require!(*owner.key != Pubkey::default(), ErrorCode::InvalidOwner);

    // ✅ Store the owner's public key
    animal.info = info;
    animal.owner_id = *owner.key; // ✅ Link animal to external owner
    
    msg!("Animal Created: Owner {:?}", animal.owner_id);
    Ok(())
}

