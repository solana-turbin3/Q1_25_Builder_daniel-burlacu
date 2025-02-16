#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Pool is locked")]
    PoolLocked,
    #[msg("Amount must be greater than 0")]
    NonPositiveAmount,
    #[msg("Slippage exceeded")]
    SlippageExceeded,
}

#[error_code]
pub enum AmmSwapError {
    #[msg("Invalid curve deposit amount")]
    InvalidDepositAmount,
    #[msg("Invalid curve withdraw amount")]
    InvalidWithdrawAmount,
}