#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

#[repr(C)] // Ensures predictable memory layout
#[account]
pub struct VetAuthority {
    pub vet_pubkey: Pubkey,    // 32 bytes
    pub animal_pubkey: Pubkey, // 32 bytes
    pub is_authorized: u8,     // 1 byte (0 = false, 1 = true)
}

// impl VetAuthority {
//     pub fn new(vet_pubkey: Pubkey, animal_pubkey: Pubkey) -> Self {
//         Self {
//             vet_pubkey,
//             animal_pubkey,
//             is_authorized: 0, // Default to "not authorized"
//         }
//     }
// }
