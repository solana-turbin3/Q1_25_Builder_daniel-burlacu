use anchor_lang::prelude::*;

#[account]
pub struct MedicalRecord {
    pub vet: Pubkey,
    pub record: Vec<u8>,
    pub date: i64,

}