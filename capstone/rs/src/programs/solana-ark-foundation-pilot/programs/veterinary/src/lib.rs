#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("BpPGFK4wmgBAiSrS3BSq3BAziYH8yCZtd6apo8Ytsdks");

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

    /// Add a new Animal Owner
    pub fn add_animal_owner(ctx: Context<AddOwner>, info: [u8; 32]) -> Result<()> {
        // Delegate logic to instructions module
        msg!("Adding Animal Owner");
        instructions::add_owner(ctx, info)
    }

    /// Add a new Animal
    pub fn add_animal(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }
    
    /// Remove an Animal
    pub fn remove_animal_owner(ctx: Context<RemoveOwner>)  -> Result<()> {
        // Delegate logic to instructions module
        msg!("Adding Animal Owner");
        instructions::remove_animal_owner(ctx)
    }

    /// Remove an Owner
    pub fn remove_animal(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }

     /// Update an Owner
     pub fn update_animal_owner(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }

    /// Update an Animal
    pub fn update_animal(ctx: Context<AddAnimal>, info: [u8; 32]) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }

    

}
