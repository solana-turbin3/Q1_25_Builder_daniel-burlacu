#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("7p5hMV3PRQvX37jKmPqyyXi3JejG2YSriMhAfbCdW8uH");

pub mod errors;
pub mod contexts;
pub mod entities;
pub mod instructions;

use crate::contexts::check_vet_auth::CheckVetAuthority;
use crate::contexts::*;
use crate::instructions::initialize;
use crate::contexts::req_auth::RequestAuthority;

#[program]
pub mod solana_animal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, info: [u8; 32]) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        initialize::initialize(ctx, info) // Call function from instructions module
    }

    pub fn request_authority(ctx: Context<RequestAuthority>) -> Result<()> {
        instructions::request_authority(ctx)
    }

    pub fn approve_or_reject_authority(ctx: Context<ApproveOrRejectAuthority>, decision: u8) -> Result<()> {
        instructions::approve_or_reject_authority(ctx, decision)
    }

    pub fn check_pending_requests<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, CheckPendingRequests<'info>>) -> Result<()> 
            where 'c: 'info {
                instructions::check_pending_requests(ctx) // ✅ Calls the function inside instructions
            }

    pub fn check_vet_authority(ctx: Context<CheckVetAuthority>) -> Result<()> {
        instructions::check_vet_authority(ctx)
    }
            
    // pub fn update_animal(ctx: Context<UpdateAnimal>, new_info: [u8; 32]) -> Result<()> {
    //     msg!("Updating Animal");
    //     // Delegate logic to instructions module
    //     instructions::update_animal(ctx, new_info)
    // }

    pub fn add_medical_record(ctx: Context<AddMedicalRecord>, record: Vec<u8>) -> Result<()> {
        msg!("Adding new medical data record PDA");
        instructions::add_medical_record(ctx, record)
    }
    
    pub fn add_behaviour_record(ctx: Context<AddBehaviourRecord>, record:  Vec<u8>) -> Result<()> {
        msg!("Adding new behavior data record PDA");
        // Delegate logic to instructions module
        instructions::add_behaviour_record(ctx, record)
    }
}