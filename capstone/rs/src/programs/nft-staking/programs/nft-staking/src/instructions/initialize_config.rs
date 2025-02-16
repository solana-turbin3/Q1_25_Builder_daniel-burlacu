use anchor_lang::prelude::*;
use anchor_spl::{token::Mint, token_2022::Token2022};

use crate::{StakeConfig, DISCRIMINATOR, REWARDS_MINT_SEED_PREFIX, STAKE_CONFIG_SEED_PREFIX};

pub fn _initialize_config(
    ctx: &mut Context<InitializeConfigAccounts>,
    input: &InitializeConfigInput,
) -> Result<()> {
    ctx.accounts.config.set_inner(StakeConfig {
        points_per_stake: input.points_per_stake,
        max_stake: input.max_stake,
        freeze_period: input.freeze_period,
        rewards_bump: ctx.bumps.rewards_mint,
        bump: ctx.bumps.config,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(input: InitializeConfigInput)]
pub struct InitializeConfigAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = DISCRIMINATOR + StakeConfig::INIT_SPACE,
        seeds = [STAKE_CONFIG_SEED_PREFIX],
        bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = admin,
        seeds = [REWARDS_MINT_SEED_PREFIX, config.key().as_ref()],
        mint::decimals = 9,
        mint::authority = admin,
        mint::token_program = token_program,
        bump
    )]
    pub rewards_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeConfigInput {
    pub points_per_stake: u8,
    pub max_stake: u8,
    pub freeze_period: u8,
}