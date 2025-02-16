use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    /// The points that the user has earned
    pub points: u32,
    /// The amount of NFTs the user has staked
    pub amount_staked: u64,
    /// The bump of the user account
    pub bump: u8,
}