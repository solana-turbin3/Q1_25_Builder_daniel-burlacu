use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("H6eB3LauYEk4RxtjNH5dwteGAH8i5qy8ukdiSvtnYmhp");

mod errors;
mod entities;
mod contexts; // Include the contexts module
mod instructions;
mod utils;

// use crate::errors::ErrorCode;
// use crate::entities::{VeterinaryCabinet, Owner, Animal}; // Import structs if needed
use crate::contexts::*; // Ensure these are correctly imported.
use crate::utils::*;
// use crate::instructions::{add_veterinary_cabinet, add_owner, add_animal, transfer_animal_ownership}; // Ensure these are correctly imported.

#[program]
pub mod veterinary_system {
    use super::*;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        msg!("Initializing Admin PDA");
        instructions::initialize_admin(ctx)
    }

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        msg!("Initializing Treasury PDA");
        instructions::initialize_treasury(ctx)
    }
    
    /// Add a new Veterinary Cabinet to the system
pub fn add_veterinary_cabinet(
    ctx: Context<AddVeterinaryCabinet>,
    info: [u8; 32],
) -> Result<()> {
    msg!("Adding Veterinary Cabinet");
    // Delegate logic to instructions module
    instructions::add_veterinary_cabinet(ctx,info)
}

    /// Add a new Animal Owner
    pub fn add_animal_owner(
        ctx: Context<AddOwner>,
        info: [u8; 32],
    ) -> Result<()> {
        // Delegate logic to instructions module
        msg!("Adding Animal Owner");
        instructions::add_owner(ctx, info)
    }

    /// Add a new Animal
    pub fn add_animal(
        ctx: Context<AddAnimal>,
        info: [u8; 32],
    ) -> Result<()> {
        msg!("Adding Animal");
        // Delegate logic to instructions module
        instructions::add_animal(ctx, info)
    }

    /// Transfer ownership of an Animal
    // pub fn transfer_animal_ownership(
    //     ctx: Context<TransferOwnership>,
    //     new_owner_id: String,
    //     veterinary_cabinet_id: String,
    // ) -> Result<()> {
    //     msg!("Transferring Animal Ownership");
    //     // Delegate logic to instructions module
    //     // mihg
    //     instructions::transfer_animal_ownership(ctx, new_owner_id,  veterinary_cabinet_id)
    // }

    pub fn mint_veterinary_cabinet_nft(
        ctx: Context<MintVeterinaryCabinetNFT>,
        metadata_uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        msg!("Minting Veterinary Cabinet NFT");
        instructions::mint_veterinary_cabinet_nft(ctx, metadata_uri, name, symbol)
    }
    
    // pub fn withdraw_from_treasury(
    //     ctx: Context<WithdrawFromTreasury>,
    //     amount: u64,
    // ) -> Result<()> {
    //     msg!("Withdrawing from Treasury");
    //     instructions::withdraw_from_treasury(ctx, amount)
    // }
    
}
