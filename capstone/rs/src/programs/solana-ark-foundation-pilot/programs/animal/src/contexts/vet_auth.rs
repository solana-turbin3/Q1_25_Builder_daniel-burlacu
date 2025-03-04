use anchor_lang::prelude::*;
use crate::entities::AuthorityRequest;
//use crate::entities::VetAuthority;
use crate::entities::Animal;
use crate::entities::VetAuthority;

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

#[derive(Accounts)]
pub struct ApproveOrRejectAuthority<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // The animal owner approving/rejecting

    /// CHECK: Explicitly pass veterinary cabinet account (previously derived from authority_request)
    pub veterinary: AccountInfo<'info>,

    /// CHECK: Explicitly pass animal account (previously derived from authority_request)
    pub animal: AccountInfo<'info>,

    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub authority_request: AccountInfo<'info>, // The pending request

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"vet_authority", veterinary.key().as_ref(), animal.key().as_ref()], // ✅ This is how it's being derived
        bump,
        space = 8 + std::mem::size_of::<VetAuthority>()
    )]
    pub vet_authority: Account<'info, VetAuthority>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckVetAuthority<'info> {
    pub vet: Signer<'info>, // ✅ Vet who is checking their access

    #[account(
        seeds = [b"vet_authority", vet.key().as_ref(), animal.key().as_ref()], // ✅ Check vet-animal relationship
        bump
    )]
    pub vet_authority: Account<'info, VetAuthority>, // ✅ If this exists, vet is authorized
    
    /// CHECK: This account is owned by another program. We manually verify it in the instruction.
    #[account(mut)]
    pub animal: AccountInfo<'info>, // ✅ The animal the vet is checking access for
}

// #[derive(Accounts)]
// pub struct RemoveAuthority<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>, // Only the owner can remove authority

//     #[account(
//         mut,
//         seeds = [b"vet_authority", owner.key().as_ref()],
//         bump
//     )]
//     pub vet_authority: Account<'info, VetAuthority>, // The vet authority PDA
// }
#[derive(Accounts)]
pub struct CheckPendingRequests<'info> {
    pub owner: Signer<'info>, // The owner who is checking

    pub system_program: Program<'info, System>,
}
