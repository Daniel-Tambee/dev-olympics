use std::borrow::BorrowMut;

use anchor_lang::prelude::*;
use crate::{ models::NftItem, NftMinted, NFT_SEED };

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
    let nft = ctx.accounts.nft.borrow_mut();
    nft.uri = file_path;
    nft.price = price;
    nft.owner = ctx.accounts.owner.key();
    nft.created_at = "".to_string();
    nft.updated_at = "".to_string();
    emit!(NftMinted {
        created_at: nft.created_at.clone(),
        mint: nft.mint,
        uri: nft.uri.clone(),
    });
    Ok(())
}
pub fn transferNft() {}
