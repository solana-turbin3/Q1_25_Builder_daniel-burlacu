use anchor_lang::prelude::*;
use crate::entities::AuthorityRequest;
use crate::entities::Animal;

#[derive(Accounts)]
pub struct RequestAuthority<'info> {
    #[account(mut)]
    pub requester: Signer<'info>, // The vet/entity requesting authority

    #[account(
        init_if_needed,
        payer = requester,
        seeds = [b"vet_authority_request", requester.key().as_ref(), animal.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<AuthorityRequest>()
    )]
    pub authority_request: Account<'info, AuthorityRequest>, // PDA for vet authority request

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub animal: Account<'info,Animal>, // The animal in question

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub owner: AccountInfo<'info>, // The owner (who will approve or reject later)

    pub system_program: Program<'info, System>,
}