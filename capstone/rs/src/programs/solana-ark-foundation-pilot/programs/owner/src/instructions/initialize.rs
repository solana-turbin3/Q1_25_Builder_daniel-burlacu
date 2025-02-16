use anchor_lang::prelude::*;
use  crate::contexts::initialize::*;

pub fn initialize(
    ctx: Context<Initialize>, 
    info: [u8; 32], // Fixed-length info for the veterinary cabinet
) -> Result<()> {
    msg!("Initializing Owner PDA");
    let owner = &mut ctx.accounts.owner; // Fix variable name
    owner.info = info; // Now modifying the correct struct
    Ok(())
}
