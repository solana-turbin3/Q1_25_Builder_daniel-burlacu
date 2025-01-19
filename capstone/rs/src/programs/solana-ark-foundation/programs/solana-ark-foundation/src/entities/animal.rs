use anchor_lang::prelude::*;

#[account]
pub struct Animal {
    pub info: [u8; 32],  // 4 bytes + metadata length
    pub owner_id: Pubkey,  // 4 bytes + owner ID length
    pub id: Pubkey, // 4 bytes + animal ID length
    pub veterinary_cabinet_id: Pubkey, // 4 bytes + cabinet ID length
}