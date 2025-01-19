use anchor_lang::prelude::*;
use crate::contexts::treasury::{InitializeTreasury, WithdrawFromTreasury};

pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
    // Initialize the treasury account
    let treasury = &mut ctx.accounts.treasury;
    treasury.balance = 0; // Start with zero balance
    treasury.admin = ctx.accounts.admin_pda.key(); // Link Treasury to Admin PDA

    Ok(())
}

pub fn withdraw_from_treasury(
    ctx: Context<WithdrawFromTreasury>,
    amount: u64,
) -> Result<()> {
    // Ensure sufficient funds in the treasury
    if ctx.accounts.treasury.balance < amount {
        return Err(error!(crate::errors::ErrorCode::InsufficientFunds));
    }

    // Perform the lamport transfer using the system program
    let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
        ctx.accounts.treasury.to_account_info().key,
        ctx.accounts.destination.key,
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.destination.to_account_info(),
        ],
    )?;

    // Update the treasury's custom balance after the transfer
    let treasury = &mut ctx.accounts.treasury; // Borrow mutably only here
    treasury.balance -= amount;

    Ok(())
}