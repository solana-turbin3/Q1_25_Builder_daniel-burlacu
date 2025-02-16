#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, TokenAccount},
    token_2022::{mint_to_checked, transfer_checked, MintToChecked, Token2022, TransferChecked},
};
use constant_product_curve::ConstantProduct;

use crate::{AmmConfig, AmmError, CONFIG_SEED_PREFIX, LP_SEED_PREFIX};

fn deposit_to_vault(
    ctx: &mut Context<DepositAccounts>,
    input: &DepositInput,
    is_x: bool,
) -> Result<()> {
    let (user_ata, vault, mint, decimals) = match is_x {
        true => (
            ctx.accounts.user_x.to_account_info(),
            ctx.accounts.vault_x.to_account_info(),
            ctx.accounts.mint_x.to_account_info(),
            ctx.accounts.mint_x.decimals,
        ),
        false => (
            ctx.accounts.user_y.to_account_info(),
            ctx.accounts.vault_y.to_account_info(),
            ctx.accounts.mint_y.to_account_info(),
            ctx.accounts.mint_y.decimals,
        ),
    };

    let user = ctx.accounts.user.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = TransferChecked {
        from: user_ata,
        to: vault,
        mint,
        authority: user,
    };

    let ctx = CpiContext::new(token_program, cpi_accounts);

    transfer_checked(ctx, input.amount, decimals)
}

fn mint_lp_to_user(ctx: &mut Context<DepositAccounts>, input: &DepositInput) -> Result<()> {
    let token_program = ctx.accounts.token_program.to_account_info();
    let mint_lp = ctx.accounts.mint_lp.to_account_info();
    let user_lp = ctx.accounts.user_lp.to_account_info();
    let config = ctx.accounts.config.to_account_info();
    let decimals = ctx.accounts.mint_lp.decimals;
    let amount = input.amount;

    let cpi_accounts = MintToChecked {
        mint: mint_lp,
        to: user_lp,
        authority: config,
    };

    let seeds = &[
        CONFIG_SEED_PREFIX,
        &ctx.accounts.config.seed.to_le_bytes(),
        &[ctx.accounts.config.bump],
    ];

    let signer_seeds = &[&seeds[..]];

    let ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);

    mint_to_checked(ctx, amount, decimals)
}

pub fn _deposit(ctx: &mut Context<DepositAccounts>, input: &DepositInput) -> Result<()> {
    require_gt!(input.amount, 0, AmmError::NonPositiveAmount);

    let is_initial_deposit =
        (ctx.accounts.mint_lp.supply | ctx.accounts.vault_x.amount | ctx.accounts.vault_y.amount)
            == 0;
    let (x, y) = match is_initial_deposit {
        true => (input.max_x, input.max_y),
        false => {
            let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                ctx.accounts.vault_x.amount,
                ctx.accounts.vault_y.amount,
                ctx.accounts.mint_lp.supply,
                input.amount,
                6,
            )
            .unwrap();
            (amounts.x, amounts.y)
        }
    };

    require_gt!(input.max_x, x, AmmError::SlippageExceeded);
    require_gt!(input.max_y, y, AmmError::SlippageExceeded);

    deposit_to_vault(ctx, input, true)?;
    deposit_to_vault(ctx, input, false)?;

    mint_lp_to_user(ctx, input)?;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_y: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_lp: Account<'info, TokenAccount>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [LP_SEED_PREFIX, config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,
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
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositInput {
    pub amount: u64,
    pub max_x: u64,
    pub max_y: u64,
}