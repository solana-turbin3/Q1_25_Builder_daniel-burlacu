use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::entities::VeterinaryCabinet;

#[derive(Accounts)]
pub struct MintVeterinaryCabinetNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Separate payer for transaction fees

    #[account(mut, has_one = id)]
    pub cabinet: Account<'info, VeterinaryCabinet>, // Veterinary cabinet account

    pub id: Signer<'info>, // Signer for validation

    /// CHECK: Master edition PDA
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: Admin PDA for signing
    #[account(seeds = [b"admin_pda".as_ref()], bump)]
    pub admin_pda: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>, // Mint account for the NFT

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, // Associated token account for mint

    /// CHECK: Metadata account (to be created)
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,          // SPL Token program
    pub system_program: Program<'info, System>,        // System program for creating accounts
}
