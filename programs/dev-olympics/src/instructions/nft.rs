use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::sysvar::clock::Clock;
use anchor_lang::solana_program::sysvar::rent::Rent;
use anchor_lang::solana_program::sysvar::Sysvar;
use crate::{ models::NftItem, NFT_SEED };

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(
        init,
        payer = issuer,
        space = std::mem::size_of::<NftItem>() + 8,
        seeds = [NFT_SEED.as_ref(), mint.key.as_ref()],
        bump
    )]
    pub nft: Account<'info, NftItem>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub issuer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    #[account(signer)]
    pub spl_token_program: AccountInfo<'info>,
    pub program_clock_sysvar: AccountInfo<'info>,
}
pub fn mint_nft(ctx: Context<MintNft>, uri: String, price: u32) -> Result<()> {
    let nft = &mut ctx.accounts.nft;
    let mint = &ctx.accounts.mint;
    let issuer = &ctx.accounts.issuer;
    let clock = Clock::get()?;

    // Initialize NFT account
    nft.mint = mint.to_account_info().key.clone();
    nft.owner = issuer.to_account_info().key.clone();
    nft.uri = uri;
    nft.price = Some(price);
    nft.created_at = clock.unix_timestamp.to_string();
    nft.updated_at = clock.unix_timestamp.to_string();

    // Mint NFT
    let mint_instruction = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint.to_account_info().key,
        &nft.to_account_info().key,
        &issuer.to_account_info().key,
        &[&issuer.to_account_info().key],
        1
    )?;
    invoke(
        &mint_instruction,
        &[
            mint.to_account_info(),
            nft.to_account_info(),
            issuer.to_account_info(),
            ctx.accounts.spl_token_program.clone(),
        ]
    )?;

    Ok(())
}
pub fn transferNft() {}
