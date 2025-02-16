use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    /// The number of points the user gets per day staked
    pub points_per_stake: u8,
    /// The maximum number of NFTs the user can stake
    pub max_stake: u8,
    /// The number of days the NFT is frozen after unstaking
    pub freeze_period: u8,
    /// The bump of the rewards mint account
    pub rewards_bump: u8,
    /// The bump of the config account
    pub bump: u8,
}