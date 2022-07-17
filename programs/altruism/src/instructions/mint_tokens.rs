use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, TokenAccount};

pub fn mint_tokens(ctx: Context<MintTokens>) -> Result<()> {
    token::mint_to(ctx.accounts.cpi_ctx(), 50)?;
    Ok(())
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut, has_one = mint)]
    pub token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}


impl<'info> MintTokens<'info> {
    pub fn cpi_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo { 
                mint: self.mint.to_account_info(),
                to: self.token.to_account_info(),
                authority: self.mint_authority.to_account_info(),
             }
        )
    }
}