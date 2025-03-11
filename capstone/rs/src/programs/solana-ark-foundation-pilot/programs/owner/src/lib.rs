#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("Bvezpv2dHr5zF9LdJDJHASiLMJGpCAVZJFy5YHY4K4WB");

pub mod errors;
pub mod contexts;
pub mod entities;
pub mod instructions;

use crate::contexts::*;
use crate::instructions::initialize;

#[program]
pub mod solana_animal_owner {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, info: [u8; 32]) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        initialize::initialize(ctx, info) // Call function from instructions module
    }

    // /// Add a new Animal
    pub fn add_animal(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }

    // /// Remove an Owner
    pub fn remove_animal(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }
    // /// Update an Animal
    pub fn update_animal(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }

        //  /// Update an Owner
    //  pub fn update_animal_owner(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
    //     msg!("Update animal owner");
    //     // Delegate logic to instructions module
    //     instructions::add_animal(ctx, info)
    // }


}