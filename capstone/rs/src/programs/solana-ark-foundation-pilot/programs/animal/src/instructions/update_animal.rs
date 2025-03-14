use anchor_lang::prelude::*;
use crate::contexts::update_animal::*;
use crate::errors::ErrorCode;
 
pub fn update_animal(
    ctx: Context<UpdateAnimal>, 
    new_info: [u8; 32], 
) -> Result<()> {
    let vet_authority = &ctx.accounts.vet_authority;

    // Ensure only authorized vets can update the animal
    require!(
        vet_authority.is_authorized == 1 && vet_authority.vet_pubkey == ctx.accounts.payer.key(), // ✅ Compare `is_authorized == 1`
        ErrorCode::UnauthorizedAccess
    );

    let animal = &mut ctx.accounts.animal;
    animal.info = new_info;

    msg!(
        "✅ Animal information updated by vet: {:?}",
        ctx.accounts.payer.key()
    );

    Ok(())
}

