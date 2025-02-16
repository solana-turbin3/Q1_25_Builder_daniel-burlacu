#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
    token_2022::Token2022,
};

use crate::{AmmConfig, CONFIG_SEED_PREFIX, DISCRIMINATOR, LP_SEED_PREFIX};

pub fn _initialize(ctx: &mut Context<InitializeAccounts>, input: &InitializeInput) -> Result<()> {
    ctx.accounts.config.set_inner(AmmConfig {
        authority: input.authority,
        seed: input.seed,
        fee: input.fee,
        locked: false,
        mint_x: ctx.accounts.mint_token_x.key(),
        mint_y: ctx.accounts.mint_token_y.key(),
        lp_bump: ctx.bumps.mint_lp_token,
        bump: ctx.bumps.config,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(input: InitializeInput)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_token_x: Account<'info, Mint>,
    pub mint_token_y: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        seeds = [LP_SEED_PREFIX, config.key.as_ref()],
        bump,
        mint::decimals = 9,
        mint::authority = config
    )]
    pub mint_lp_token: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_token_x,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_token_x: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_token_y,
        associated_token::authority = config,
        associated_token::token_program = token_program
    )]
    pub vault_token_y: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user,
        seeds = [CONFIG_SEED_PREFIX, input.seed.to_le_bytes().as_ref()],
        bump,
        space = DISCRIMINATOR + AmmConfig::INIT_SPACE
    )]
    pub config: Account<'info, AmmConfig>,

    // Program accounts
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeInput {
    pub seed: u64,
    pub fee: u16,
    pub authority: Option<Pubkey>,
}