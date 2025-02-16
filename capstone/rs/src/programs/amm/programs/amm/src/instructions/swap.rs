#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
    token_2022::{transfer_checked, Token2022, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{AmmConfig, AmmError, AmmSwapError, CONFIG_SEED_PREFIX};

fn deposit_to_user(ctx: &mut Context<SwapAccounts>, is_x: bool, amount: u64) -> Result<()> {
    let (vault, user_ata, mint, decimals) = match is_x {
        true => (
            ctx.accounts.vault_x.to_account_info(),
            ctx.accounts.user_x.to_account_info(),
            ctx.accounts.mint_x.to_account_info(),
            ctx.accounts.mint_x.decimals,
        ),
        false => (
            ctx.accounts.vault_y.to_account_info(),
            ctx.accounts.user_y.to_account_info(),
            ctx.accounts.mint_y.to_account_info(),
            ctx.accounts.mint_y.decimals,
        ),
    };
    let config = ctx.accounts.config.to_account_info();

    let token_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = TransferChecked {
        from: vault,
        mint,
        to: user_ata,
        authority: config,
    };
    let cpi_ctx = CpiContext::new(token_program, cpi_accounts);

    transfer_checked(cpi_ctx, amount, decimals)
}

fn withdraw_from_vault(ctx: &mut Context<SwapAccounts>, is_x: bool, amount: u64) -> Result<()> {
    // since this is a swap, if it's x, then we're withdrawing y, and vice versa
    let (vault, user_ata, mint, decimals) = match is_x {
        true => (
            ctx.accounts.vault_y.to_account_info(),
            ctx.accounts.user_y.to_account_info(),
            ctx.accounts.mint_y.to_account_info(),
            ctx.accounts.mint_y.decimals,
        ),
        false => (
            ctx.accounts.vault_x.to_account_info(),
            ctx.accounts.user_x.to_account_info(),
            ctx.accounts.mint_x.to_account_info(),
            ctx.accounts.mint_x.decimals,
        ),
    };

    let token_program = ctx.accounts.token_program.to_account_info();
    let config = ctx.accounts.config.to_account_info();
    let cpi_accounts = TransferChecked {
        from: vault,
        mint,
        to: user_ata,
        authority: config,
    };
    let seeds = &[
        CONFIG_SEED_PREFIX,
        &ctx.accounts.config.seed.to_le_bytes(),
        &[ctx.accounts.config.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

    transfer_checked(cpi_ctx, amount, decimals)
}

pub fn _swap(ctx: &mut Context<SwapAccounts>, input: &SwapInput) -> Result<()> {
    require_gt!(input.amount, 0, AmmError::NonPositiveAmount);

    let mut curve = ConstantProduct::init(
        ctx.accounts.vault_x.amount,
        ctx.accounts.vault_y.amount,
        ctx.accounts.vault_x.amount,
        ctx.accounts.config.fee,
        None,
    )
    .unwrap();

    let liquidity_pair = match input.is_x {
        true => LiquidityPair::X,
        false => LiquidityPair::Y,
    };

    let (deposit, withdraw) = {
        let amounts = curve.swap(liquidity_pair, input.amount, input.min).unwrap();
        (amounts.deposit, amounts.withdraw)
    };

    require_neq!(deposit, 0, AmmSwapError::InvalidDepositAmount);
    require_neq!(withdraw, 0, AmmSwapError::InvalidWithdrawAmount);

    deposit_to_user(ctx, input.is_x, deposit)?;
    withdraw_from_vault(ctx, input.is_x, withdraw)?;

    Ok(())
}

#[derive(Accounts)]
pub struct SwapAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_y: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [CONFIG_SEED_PREFIX, config.seed.to_le_bytes().as_ref()],
        bump = config.bump,
        constraint = !config.locked @ AmmError::PoolLocked
    )]
    pub config: Account<'info, AmmConfig>,

    // Program accounts
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapInput {
    pub amount: u64,
    pub min: u64,
    pub is_x: bool,
}