use anchor_lang::prelude::*;

#[account]
pub struct VeterinaryCabinet  {
    pub id: Pubkey,       // 32 
    pub info: [u8; 32],       // 4 bytes + name length
}
