use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::FreezeDelegatedAccountCpiBuilder, MasterEditionAccount,
        Metadata, MetadataAccount,
    },
    token::{Mint, TokenAccount},
    token_2022::{approve_checked, ApproveChecked, Token2022},
};

use crate::{
    StakeAccount, StakeConfig, StakeErrors, UserAccount, DISCRIMINATOR, EDITION_SEED_SUFFIX,
    METADATA_SEED_PREFIX, STAKE_ACCOUNT_SEED_PREFIX, STAKE_CONFIG_SEED_PREFIX,
    USER_ACCOUNT_SEED_PREFIX,
};

/// Approve the stake account to transfer the NFT
pub fn approve_nft_stake(ctx: &mut Context<StakeAccounts>) -> Result<()> {
    let decimals = ctx.accounts.mint.decimals;
    let token_program = ctx.accounts.token_program.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let mint_ata = ctx.accounts.mint_ata.to_account_info();
    let stake_account = ctx.accounts.stake_account.to_account_info();
    let user = ctx.accounts.user.to_account_info();

    let cpi_accounts = ApproveChecked {
        to: mint_ata,
        delegate: stake_account,
        authority: user,
        mint,
    };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);

    approve_checked(cpi_ctx, 1, decimals)
}

/// Freeze the NFT while staked
pub fn freeze_nft_stake(ctx: &mut Context<StakeAccounts>) -> Result<()> {
    let delegate = &ctx.accounts.stake_account.to_account_info();
    let token_account = &ctx.accounts.mint_ata.to_account_info();
    let edition = &ctx.accounts.edition.to_account_info();
    let mint = &ctx.accounts.mint.to_account_info();
    let token_program = &ctx.accounts.token_program.to_account_info();
    let metadata_program = &ctx.accounts.metadata_program.to_account_info();

    let seeds = &[
        STAKE_ACCOUNT_SEED_PREFIX,
        ctx.accounts.mint.to_account_info().key.as_ref(),
        ctx.accounts.config.to_account_info().key.as_ref(),
        &[ctx.accounts.stake_account.bump],
    ];
    let signers_seeds = &[&seeds[..]];

    FreezeDelegatedAccountCpiBuilder::new(metadata_program)
        .delegate(delegate)
        .token_account(token_account)
        .edition(edition)
        .mint(mint)
        .token_program(token_program)
        .invoke_signed(signers_seeds)?;

    Ok(())
}

pub fn _stake(ctx: &mut Context<StakeAccounts>) -> Result<()> {
    ctx.accounts.stake_account.set_inner(StakeAccount {
        bump: ctx.bumps.stake_account,
        owner: ctx.accounts.user.key(),
        mint: ctx.accounts.mint.key(),
        updated_at: Clock::get()?.unix_timestamp,
    });

    approve_nft_stake(ctx)?;
    freeze_nft_stake(ctx)?;

    ctx.accounts
        .user_account
        .amount_staked
        .checked_add(1)
        .unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct StakeAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = DISCRIMINATOR + StakeAccount::INIT_SPACE,
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
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref() @ StakeErrors::InvalidCollection,
        constraint = metadata.collection.as_ref().unwrap().verified @ StakeErrors::CollectionNotVerified,
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