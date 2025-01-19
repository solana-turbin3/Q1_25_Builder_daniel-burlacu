use anchor_lang::prelude::*;

#[account]
pub struct Owner {
    pub info: [u8; 32],       // 4 bytes + name length
    pub veterinary_cabinet_id: Pubkey,  // 4 bytes + animal ID length
    pub id: Pubkey,     // 32 bytes
}
