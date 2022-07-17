use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

pub fn create_token_account(_ctx: Context<CreateTokenAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = authority
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
