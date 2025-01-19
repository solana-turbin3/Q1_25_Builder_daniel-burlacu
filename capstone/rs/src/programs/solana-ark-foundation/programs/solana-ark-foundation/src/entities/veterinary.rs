use anchor_lang::prelude::*;

#[account]
pub struct VeterinaryCabinet {
    pub id: Pubkey,       // 32 
    pub info: [u8; 32],       // 4 bytes + name length
    //pub id: [u8; 32],         // 4 bytes + id length
    // pub phone: String,        // 4 bytes + phone length
    // pub country: String,      // 4 bytes + country length
    // pub town: String,         // 4 bytes + town length
    // pub postcode: String,     // 4 bytes + postcode length
    // pub address: String,      // 4 bytes + address length
    pub expire_date: i64,     // 8 bytes
    pub trust_score: u8,      // 1 byte
}
