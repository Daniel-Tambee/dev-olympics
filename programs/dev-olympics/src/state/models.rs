use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct NftItem {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub uri: String,
    pub price: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
}

#[account]
#[derive(Default)]
pub struct Vault {
    pub items: Option<Vec<Pubkey>>,
    pub updated_at: String,
}

#[account]
#[derive(Default)]
pub struct Collection {
    pub mint: Pubkey,
    pub issuer: Pubkey,
    pub name: Option<String>,
    pub buyer: Option<Pubkey>,
    pub items: Vec<Pubkey>,
    pub price: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
}
