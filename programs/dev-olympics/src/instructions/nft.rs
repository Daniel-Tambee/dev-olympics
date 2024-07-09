use std::borrow::BorrowMut;

use anchor_lang::prelude::*;
use chrono::prelude::*;
use crate::{ models::NftItem, NFT_SEED };

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(
        init,
        payer = owner,
        space = std::mem::size_of::<NftItem>() + 8,
        seeds = [NFT_SEED.as_bytes(), owner.key().as_ref(), b"some_cool_value"],
        bump
    )]
    pub nft: Account<'info, NftItem>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn mint_nft(ctx: Context<MintNft>, price: Option<u32>, file_path: String) -> Result<()> {
    let now: DateTime<Utc> = Utc::now();

    let nft = ctx.accounts.nft.borrow_mut();
    nft.uri = file_path;
    nft.price = price;
    nft.owner = ctx.accounts.owner.key();
    nft.created_at = now.to_rfc3339();
    nft.updated_at = now.to_rfc3339();
    Ok(())
}
pub fn transferNft() {}
