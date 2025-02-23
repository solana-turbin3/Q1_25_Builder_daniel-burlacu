#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("4fyNSjyKdfB47hR2DPX1zwFwbox2b58PAermmytFGdtB");

pub mod errors;
pub mod contexts;
pub mod entities;
pub mod instructions;

use crate::contexts::*;
use crate::instructions::initialize;

#[program]
pub mod solana_vet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, info: [u8; 32]) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        initialize::initialize(ctx, info) // Call function from instructions module
    }

    pub fn update_animal(ctx: Context<UpdateAnimal>, new_info: [u8; 32]) -> Result<()> {
        msg!("Updating Animal");
        // Delegate logic to instructions module
        instructions::update_animal(ctx, new_info)
    }

    pub fn add_medical_record(ctx: Context<AddMedicalRecord>, record: Vec<u8>) -> Result<()> {
        msg!("Adding new medical data record PDA");
        // Delegate logic to instructions module
        instructions::add_medical_record(ctx, record)
    }

    pub fn add_behaviour_record(ctx: Context<AddBehaviourRecord>, record:  Vec<u8>) -> Result<()> {
        msg!("Adding new behavior data record PDA");
        // Delegate logic to instructions module
        instructions::add_behaviour_record(ctx, record)
    }
    
    pub fn add_owner(ctx: Context<AddOwner>, owner_id: Pubkey, info: [u8; 32]) -> Result<()> {
        msg!("Adding new owner data record PDA");
        // Delegate logic to instructions module
        instructions::add_owner(ctx, owner_id, info)
    }

    pub fn remove_authority(ctx: Context<RemoveAuthority>) -> Result<()> {
        msg!("Revoking Vet Authority");
        instructions::remove_authority(ctx)
    }

}