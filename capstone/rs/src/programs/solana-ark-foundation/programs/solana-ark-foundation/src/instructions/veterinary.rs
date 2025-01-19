 /// Add a new Veterinary Cabinet to the system
 use anchor_lang::prelude::*;
 use anchor_lang::solana_program::system_instruction;
 use crate::contexts::veterinary::AddVeterinaryCabinet;
 use crate::errors::ErrorCode;
 use chrono::{TimeZone,Datelike};
 
 pub const JOIN_FEE: u64 = 800_000; // 0.8 SOL (in lamports)
 
 fn is_leap_year(year: i32) -> bool {
   (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
 }
 
 pub fn add_veterinary_cabinet(
     ctx: Context<AddVeterinaryCabinet>,
     info: [u8; 32], // Fixed-length info for the veterinary cabinet
 ) -> Result<()> {
     let cabinet = &mut ctx.accounts.cabinet;
     let payer = &ctx.accounts.payer;
     let treasury = &mut ctx.accounts.treasury;
 
     // Validate the provided info input
     if info.iter().all(|&byte| byte == 0) {
         return err!(ErrorCode::InvalidMetadata); // Ensure info is not empty
     }
 
     // Ensure the payer has enough lamports to cover the JOIN_FEE
     if **payer.lamports.borrow() < JOIN_FEE {
         return err!(ErrorCode::InsufficientFee);
     }
 
     // Transfer JOIN_FEE from payer to the treasury account
     let transfer_ix = system_instruction::transfer(
         payer.key,         // Source account
         &treasury.key(),   // Destination account (Treasury PDA)
         JOIN_FEE,          // Amount to transfer
     );
 
     // Use invoke to securely execute the transfer instruction
     anchor_lang::solana_program::program::invoke(
         &transfer_ix,
         &[
             payer.to_account_info(),
             treasury.to_account_info(),
         ],
     )?;
     // Calculate the exact year length
     let current_time = Clock::get()?.unix_timestamp;
     let current_year = chrono::Utc.timestamp(current_time, 0).year(); // Requires chrono crate
     let year_length_in_seconds = if is_leap_year(current_year) {
         366 * 24 * 60 * 60 // Leap year
     } else {
         365 * 24 * 60 * 60 // Non-leap year
     };
 
     // Update the treasury's custom balance field (if used)
     treasury.balance += JOIN_FEE;
 
     // Initialize the Veterinary Cabinet account
     cabinet.id = payer.key();
     cabinet.info = info; // Assign the provided info
     cabinet.trust_score = 0; // Default trust score
 
     // Log success
     msg!("Veterinary Cabinet added successfully: {}", cabinet.key());
     msg!("JOIN_FEE of {} transferred to treasury: {}", JOIN_FEE, treasury.key());
 
     Ok(())
 }