use anchor_lang::prelude::*;

#[event]
pub struct NftMinted {
    pub mint: Pubkey,
    pub uri: String,
    pub created_at: u32,
}

#[event]
pub struct CollectionMinted {
    pub mint: Pubkey,
    pub name: Pubkey,
    pub created_at: u32,
}

#[event]
pub struct NftTransfered {
    pub mint: Pubkey,
    pub new_owner: Pubkey,
    pub updated_at: u32,
}

#[event]
pub struct CollectionTransfered {
    pub mint: Pubkey,
    pub buyer: Pubkey,
    pub updated_at: u32,
}

#[event]
pub struct VaultIncrement {
    pub mint: Pubkey,
    pub updated_at: u32,
}

#[event]
pub struct VaultDecrement {
    pub mint: Pubkey,
    pub updated_at: u32,
}
