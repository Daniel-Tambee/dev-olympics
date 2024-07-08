use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("NFT already in vault")]
    DuplicateItem,
    #[msg("NFT not in the vault")]
    ItemNotFound,
    #[msg("You dont own this")]
    NotTheOwner,
    #[msg("You dont have enough sol")]
    LittleLamports,
}
