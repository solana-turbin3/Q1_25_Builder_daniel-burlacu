use anchor_lang::prelude::*;
use crate::entities::Animal;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The person deploying the program

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"animal", payer.key().as_ref()], // Unique PDA for the animal
        bump,
        space = 8 + std::mem::size_of::<Animal>()
    )]
    pub animal: Account<'info, Animal>,

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
