use anchor_lang::{prelude::*, solana_program::clock::SECONDS_PER_DAY};
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::ThawDelegatedAccountCpiBuilder, MasterEditionAccount,
        Metadata, MetadataAccount,
    },
    token::{Mint, TokenAccount},
    token_2022::{revoke, Revoke, Token2022},
};

use crate::{
    StakeAccount, StakeConfig, UnstakeErrors, UserAccount, EDITION_SEED_SUFFIX,
    METADATA_SEED_PREFIX, STAKE_ACCOUNT_SEED_PREFIX, STAKE_CONFIG_SEED_PREFIX,
    USER_ACCOUNT_SEED_PREFIX,
};

pub fn get_days_since_stake(stake_account: &StakeAccount) -> u8 {
    Clock::get()
        .unwrap()
        .unix_timestamp
        .checked_sub(stake_account.updated_at)
        .unwrap()
        .checked_div(SECONDS_PER_DAY as i64)
        .unwrap() as u8
}

pub fn get_points_earned(days_staked: u8, points_per_stake: u8) -> u32 {
    points_per_stake.checked_mul(days_staked).unwrap() as u32
}

/// Remove delegation of the NFT
pub fn thaw_nft(ctx: &Context<UnstakeAccounts>) -> Result<()> {
    let mint = &ctx.accounts.mint.to_account_info();
    let stake_account = &ctx.accounts.stake_account.to_account_info();
    let mint_ata = &ctx.accounts.mint_ata.to_account_info();
    let edition = &ctx.accounts.edition.to_account_info();
    let token_program = &ctx.accounts.token_program.to_account_info();
    let metadata_program = &ctx.accounts.metadata_program.to_account_info();
    let config = &ctx.accounts.config.to_account_info();

    let seeds = &[
        STAKE_ACCOUNT_SEED_PREFIX,
        mint.key.as_ref(),
        config.key.as_ref(),
        &[ctx.bumps.stake_account],
    ];
    let signers_seeds = &[&seeds[..]];

    ThawDelegatedAccountCpiBuilder::new(metadata_program)
        .delegate(stake_account)
        .token_account(mint_ata)
        .edition(edition)
        .mint(mint)
        .token_program(token_program)
        .invoke_signed(signers_seeds)?;

    Ok(())
}

pub fn revoke_delegation(ctx: &Context<UnstakeAccounts>) -> Result<()> {
    let source = ctx.accounts.mint_ata.to_account_info();
    let authority = ctx.accounts.user.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();

    let cpi_accounts = Revoke { source, authority };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);

    revoke(cpi_ctx)
}

pub fn _unstake(ctx: &mut Context<UnstakeAccounts>) -> Result<()> {
    let days_since_stake = get_days_since_stake(&ctx.accounts.stake_account);

    require!(
        days_since_stake >= ctx.accounts.config.freeze_period,
        UnstakeErrors::NotEnoughDaysStaked
    );

    let points = get_points_earned(days_since_stake, ctx.accounts.config.points_per_stake);
    ctx.accounts.user_account.points = ctx
        .accounts
        .user_account
        .points
        .checked_add(points)
        .unwrap();

    thaw_nft(ctx)?;
    revoke_delegation(ctx)?;

    ctx.accounts.user_account.amount_staked = ctx
        .accounts
        .user_account
        .amount_staked
        .checked_sub(1)
        .unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [STAKE_ACCOUNT_SEED_PREFIX, mint.key().as_ref(), config.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED_PREFIX, user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub mint: Account<'info, Mint>,
    pub collection: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            METADATA_SEED_PREFIX,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            METADATA_SEED_PREFIX,
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            EDITION_SEED_SUFFIX,
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,
    #[account(
        seeds = [STAKE_CONFIG_SEED_PREFIX],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    /// Program accounts
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}