// contexts/initialize.rs
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::entities::behaviour_record::BehaviourRecord;

#[derive(Accounts)]
pub struct AddBehaviourRecord<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [b"behaviour_record", owner.key().as_ref()],
        bump,
        space = 8 + 512
    )]
    pub behaviour_record: Account<'info, BehaviourRecord>, // New medical record

    pub system_program: Program<'info, System>,
}
