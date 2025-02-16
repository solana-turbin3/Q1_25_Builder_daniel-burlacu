use anchor_lang::prelude::*;
use crate::contexts::behaviour_record::AddBehaviourRecord;


pub fn add_behaviour_record(
  ctx: Context<AddBehaviourRecord>,
  record: [u8; 32],
) -> Result<()> {
    msg!("Adding new behaviour data record PDA");
    let behaviour_record = &mut ctx.accounts.behaviour_record; // Fix variable name
    behaviour_record.record = record; // Now modifying the correct struct
    Ok(())
}
