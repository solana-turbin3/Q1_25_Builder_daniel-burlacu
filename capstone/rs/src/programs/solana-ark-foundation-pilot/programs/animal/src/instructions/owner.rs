use anchor_lang::prelude::*;
use  crate::contexts::owner::*;

pub fn add_owner(
  ctx: Context<AddOwner>,
  owner_id: Pubkey,
  info: [u8; 32]
) -> Result<()> {
    msg!("Adding new behaviour data record PDA");
    let owner = &mut ctx.accounts.owner; // Fix variable name
    owner.info = info;
    owner.owner_id = owner_id.key(); // Now modifying the correct struct

    Ok(())
}