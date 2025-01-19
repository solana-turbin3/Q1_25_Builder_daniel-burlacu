use anchor_lang::prelude::*;
use crate::contexts::admin::InitializeAdmin;
use crate::constants::ADMIN_SEED;

pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
    if ctx.accounts.admin_pda.key != Pubkey::default() {
        return Err(error!(crate::errors::ErrorCode::AdminAlreadyInitialized));
    }

    let (admin_pda, _bump) = Pubkey::find_program_address(&[ADMIN_SEED], ctx.program_id);
    let admin_account = &mut ctx.accounts.admin_pda;
    
    // Ensure this function is not called again if the Admin PDA already exists
    
  
    // Set the derived `admin_pda` as the key for the account.
    admin_account.key = admin_pda;

    // Set the admin wallet public key
    admin_account.admin = ctx.accounts.payer.key();
    
    msg!("Admin AccountKey: {:?}", admin_account.key);
    msg!("Admin Wallet: {:?}", admin_account.admin);

    Ok(())
}

