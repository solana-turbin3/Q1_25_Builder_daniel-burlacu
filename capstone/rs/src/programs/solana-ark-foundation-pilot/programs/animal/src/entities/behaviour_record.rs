#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

#[account]
pub struct BehaviourRecord {
    pub record: Vec<u8>,  // 4 bytes + metadata length
    pub owner_id: Pubkey,  // 4 bytes + owner ID length
    pub id: Pubkey, // 4 bytes + animal ID length
}