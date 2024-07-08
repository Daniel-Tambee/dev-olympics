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
    pub createdAt: u32,
}

#[event]
pub struct NftTransfered {
    pub mint: Pubkey,
    pub newOwner: Pubkey,
    pub updatedAt: u32,
}

#[event]
pub struct CollectionTransfered {
    pub mint: Pubkey,
    pub buyer: Pubkey,
    pub updated_at: u32,
}

#[event]
pub struct vaultIncrement {
    pub mint: Pubkey,
    pub updatedAt: u32,
}

#[event]
pub struct vaultDecrement {
    pub mint: Pubkey,
    pub updatedAt: u32,
}
