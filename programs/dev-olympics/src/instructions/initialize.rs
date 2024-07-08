use anchor_lang::prelude::*;

use crate::{ models::Vault, SEED };

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = std::mem::size_of::<Vault>() + 8,
        payer = signer,
        seeds = [SEED.as_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {{:?}}",);
    Ok(())
}
