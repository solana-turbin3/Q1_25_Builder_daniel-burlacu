use anchor_lang::prelude::*; 
use crate::contexts::check_vet_auth::CheckVetAuthority;
use crate::contexts::CheckPendingRequests;
use crate::entities::AuthorityRequest;

pub fn check_vet_authority(ctx: Context<CheckVetAuthority>) -> Result<()> {
    let vet_authority = &ctx.accounts.vet_authority;

    if vet_authority.is_authorized == 1 { // ✅ Now explicitly checking against `1`
        msg!(
            "✅ Vet {:?} is authorized for animal {:?}",
            vet_authority.vet_pubkey,
            vet_authority.animal_pubkey
        );
    } else {
        msg!(
            "❌ Vet {:?} is NOT authorized for animal {:?}",
            vet_authority.vet_pubkey,
            vet_authority.animal_pubkey
        );
    }
    
    Ok(())
}

pub fn check_pending_requests<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CheckPendingRequests<'info>>,
) -> Result<()>
where
    'c: 'info,
{
    msg!("🔍 Scanning for pending requests...");

    for account_info in ctx.remaining_accounts.iter() {
        let authority_request = Account::<AuthorityRequest>::try_from(account_info);

        match authority_request {
            Ok(request) if request.status == 0 => {
                msg!("✅ Pending Request Found: {:?}", request.key());
            }
            Err(_) => {
                msg!("❌ Skipping invalid account {:?}", account_info.key);
            }
            _ => {} // Ignore non-pending requests
        }
    }

    Ok(())
}