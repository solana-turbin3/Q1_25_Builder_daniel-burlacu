use anchor_lang::prelude::*;
use crate::entities::{Animal, Owner, VetAuthority};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The person deploying the program

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"animal", payer.key().as_ref()], // Unique PDA for the animal
        bump,
        space = 8 + std::mem::size_of::<Animal>() 
    )]
    pub animal: Account<'info, Animal>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"owner", payer.key().as_ref()], // Ensuring a unique Owner PDA
        bump,
        space = 8 + std::mem::size_of::<Owner>() 
    )]
    pub owner: Account<'info, Owner>, // Owner is created at initialization

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"vet_authority", payer.key().as_ref()], // VetAuthority PDA
        bump,
        space = 8 + std::mem::size_of::<VetAuthority>()
    )]
    pub vet_authority: Account<'info, VetAuthority>, // Authority PDA

    pub system_program: Program<'info, System>, 
}

pub fn initialize(ctx: Context<Initialize>, info: [u8; 32]) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    owner.owner_id = ctx.accounts.payer.key();
    owner.info = info;

    let authority = &mut ctx.accounts.vet_authority;
    authority.is_authorized = false;
    authority.vet_pubkey = Pubkey::default();

    msg!("Initialized Owner & Vet Authority");
    Ok(())
}
