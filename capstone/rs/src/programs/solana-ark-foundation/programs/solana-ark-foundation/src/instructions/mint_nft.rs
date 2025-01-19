use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use mpl_token_metadata::instructions::CreateV1Builder;
use mpl_token_metadata::types::{TokenStandard, PrintSupply};
use crate::contexts::mint_nft::MintVeterinaryCabinetNFT;
use crate::errors::ErrorCode;

pub fn mint_veterinary_cabinet_nft(
    ctx: Context<MintVeterinaryCabinetNFT>,
    metadata_uri: String,
    name: String,
    symbol: String,
) -> Result<()> {
    msg!("Minting Veterinary Cabinet NFT");

    // Validate inputs
    if metadata_uri.len() > 200 {
        return err!(ErrorCode::InvalidMetadataURI);
    }
    if name.len() > 32 {
        return err!(ErrorCode::InvalidName);
    }
    if symbol.len() > 10 {
        return err!(ErrorCode::InvalidSymbol);
    }

    // Validate cabinet ownership
    if ctx.accounts.cabinet.id != ctx.accounts.payer.key() {
        return err!(ErrorCode::UnauthorizedAccess);
    }

    // Mint the NFT
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.admin_pda.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, 1)?; // Mint 1 token

    // Prepare metadata creation instruction
    let instruction = CreateV1Builder::new()
        .metadata(ctx.accounts.metadata_account.key())
        .master_edition(Some(ctx.accounts.master_edition.key()))
        .mint(ctx.accounts.mint.key(), true)
        .authority(ctx.accounts.admin_pda.key())
        .payer(ctx.accounts.payer.key())
        .update_authority(ctx.accounts.admin_pda.key(), true)
        .is_mutable(true)
        .primary_sale_happened(false)
        .name(name)
        .symbol(symbol)
        .uri(metadata_uri)
        .seller_fee_basis_points(500)
        .token_standard(TokenStandard::NonFungible)
        .print_supply(PrintSupply::Zero)
        .instruction();

    // CPI Invocation for metadata creation
    anchor_lang::solana_program::program::invoke_signed(
        &instruction,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.admin_pda.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ],
        &[&[
            b"admin_pda".as_ref(),
            &[ctx.bumps.admin_pda],
        ]],
    )?;

    msg!("Veterinary Cabinet NFT minted successfully.");

    Ok(())
}
