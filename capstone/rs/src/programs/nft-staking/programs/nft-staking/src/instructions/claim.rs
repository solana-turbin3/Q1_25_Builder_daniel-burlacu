use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
    token_2022::{mint_to_checked, MintToChecked, Token2022},
};

use crate::{
    state::{StakeConfig, UserAccount},
    REWARDS_MINT_SEED_PREFIX, STAKE_CONFIG_SEED_PREFIX, USER_ACCOUNT_SEED_PREFIX,
};

pub fn mint_rewards(ctx: &mut Context<ClaimAccounts>) -> Result<()> {
    let token_program = ctx.accounts.token_program.to_account_info();
    let rewards_mint = ctx.accounts.rewards_mint.to_account_info();
    let rewards_ata = ctx.accounts.rewards_ata.to_account_info();
    let config = ctx.accounts.config.to_account_info();

    let config_bump = ctx.accounts.config.bump;
    let seeds = &[STAKE_CONFIG_SEED_PREFIX, &[config_bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = MintToChecked {
        mint: rewards_mint,
        to: rewards_ata,
        authority: config,
    };
    let cpi_context = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

    let decimals = ctx.accounts.rewards_mint.decimals;
    let points = ctx.accounts.user_account.points as u64;
    let rewards = points * 10_u64.checked_pow(decimals as u32).unwrap();

    mint_to_checked(cpi_context, rewards, decimals)
}

pub fn _claim(ctx: &mut Context<ClaimAccounts>) -> Result<()> {
    mint_rewards(ctx)?;

    // Reset user points for next claims
    ctx.accounts.user_account.points = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED_PREFIX, user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [STAKE_CONFIG_SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [REWARDS_MINT_SEED_PREFIX, config.key().as_ref()],
        bump = config.rewards_bump,
    )]
    pub rewards_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    // Program accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}