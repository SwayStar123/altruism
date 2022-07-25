use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint};

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.state.alt_sol_mint_pubkey = ctx.accounts.mint.key();
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 32 + 8,
    )]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

// 32
#[account]
pub struct State {
    pub alt_sol_mint_pubkey: Pubkey, // 32
}