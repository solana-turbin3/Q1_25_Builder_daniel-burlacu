/// Add a new Animal Owner
use anchor_lang::prelude::*;
use crate::contexts::owner::AddOwner;
use crate::errors::ErrorCode;

pub fn add_owner(
    ctx: Context<AddOwner>,
    info: [u8; 32],
) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let cabinet = &mut ctx.accounts.cabinet;
    let payer = &ctx.accounts.payer;
   
    // Ensure the caller is a recognized veterinary cabinet
    if cabinet.id != ctx.accounts.payer.key() {
        return err!(ErrorCode::UnauthorizedAccess);
    }

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