use anchor_lang::prelude::*;

use crate::{UserAccount, DISCRIMINATOR, USER_ACCOUNT_SEED_PREFIX};

pub fn _initialize_user(ctx: &mut Context<InitializeUserAccounts>) -> Result<()> {
    ctx.accounts.user_account.set_inner(UserAccount {
        bump: ctx.bumps.user_account,
        points: 0,
        amount_staked: 0,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUserAccounts<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = DISCRIMINATOR + UserAccount::INIT_SPACE,
        seeds = [USER_ACCOUNT_SEED_PREFIX, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}