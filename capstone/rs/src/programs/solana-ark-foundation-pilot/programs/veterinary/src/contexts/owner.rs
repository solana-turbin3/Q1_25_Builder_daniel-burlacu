use anchor_lang::prelude::*;
use crate::entities::{ Owner, VeterinaryCabinet};

#[derive(Accounts)]
pub struct AddOwner<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds =[b"owner".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump,
        space = 8 + 256 // Account discriminator + VeterinaryCabinet fields
    )]
    pub owner: Account<'info, Owner>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateOwner<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>,

    #[account(
        mut,
        seeds =[b"owner".as_ref(), payer.key().as_ref()], // Seed for VeterinaryCabinet PDA
        bump 
    )]
    pub owner: Account<'info, Owner>,
    
    #[account(mut)]
    pub payer: Signer<'info>
}

#[derive(Accounts)]
pub struct RemoveOwner<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>,

    #[account(
        mut,
        close = payer,  // Closes the PDA and returns any remaining lamports to payer
        seeds = [b"owner".as_ref(), payer.key().as_ref()],
        bump
    )]
    pub owner: Account<'info, Owner>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

