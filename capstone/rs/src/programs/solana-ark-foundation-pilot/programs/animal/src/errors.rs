#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Caller wallet does not match the cabinet owner.")]
    UnauthorizedAccess,
    #[msg("Invalid metadata: Metadata URI is required.")]
    InvalidMetadata,
    #[msg("Insufficient fee for joining the program.")]
    InsufficientFee,
    #[msg("Too much fee for joining the program.")]
    TooMuchFee,
    #[msg("Insufficient funds in the treasury.")]
    InsufficientFunds,
    #[msg("Admin PDA already initialized.")]
    AdminAlreadyInitialized,
    #[msg("Treasury PDA is already initialized.")]
    TreasuryAlreadyInitialized,
    #[msg("Provided treasury account is invalid.")]
    InvalidTreasuryAccount,
    #[msg("Missing signature.")]
    MissingSignature,
    #[msg("Invalid symbol.")]
    InvalidSymbol,
    #[msg("Invalid name.")]
    InvalidName,
    #[msg("Invalid metadadataURI.")]
    InvalidMetadataURI,    
}
