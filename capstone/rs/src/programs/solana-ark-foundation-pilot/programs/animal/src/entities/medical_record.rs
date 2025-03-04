#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

#[account]
pub struct MedicalRecord {
    pub animal_id: Pubkey, // ✅ Link record to a specific animal
    pub vet: Pubkey,       // ✅ Vet who added the record
    pub record: Vec<u8>,   // ✅ The medical data
    pub date: i64,         // ✅ Timestamp
}
