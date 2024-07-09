pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use events::*;
declare_id!("8GVRTVQc8vT2eUmDT3bCRNZwx3eN2gGrXZ46x7N6xQ62");

#[program]
pub mod dev_olympics {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
    pub fn mint_nft(ctx: Context<MintNft>, price: Option<u32>, file_path: String) -> Result<()> {
        instructions::mint_nft(ctx, price, file_path)
    }
}
