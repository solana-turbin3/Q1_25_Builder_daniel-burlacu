#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("BDYbMNRm7dY4Ni45sFMBkZWZhqU1oNivngobu8aqcWQ");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        mut ctx: Context<InitializeConfigAccounts>,
        input: InitializeConfigInput,
    ) -> Result<()> {
        _initialize_config(&mut ctx, &input)
    }

    pub fn initialize_user(mut ctx: Context<InitializeUserAccounts>) -> Result<()> {
        _initialize_user(&mut ctx)
    }

    pub fn stake(mut ctx: Context<StakeAccounts>) -> Result<()> {
        _stake(&mut ctx)
    }

    pub fn unstake(mut ctx: Context<UnstakeAccounts>) -> Result<()> {
        _unstake(&mut ctx)
    }

    pub fn claim(mut ctx: Context<ClaimAccounts>) -> Result<()> {
        _claim(&mut ctx)
    }
}