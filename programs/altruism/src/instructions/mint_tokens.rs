use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, TokenAccount};

use crate::instructions::initialize::Vault;

pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // creating SOL transfer instruction
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        ctx.accounts.mint_authority.signer_key().unwrap(), 
        &ctx.accounts.vault.key(), 
        amount
    );

    // invoking SOL transfer instruction
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            // from
            ctx.accounts.mint_authority.to_account_info(),
            // to
            ctx.accounts.vault.to_account_info(),
        ]
    )?;

    token::mint_to(ctx.accounts.cpi_ctx(), amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut, has_one = mint)]
    pub token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut, seeds=[b"vault"], bump = vault.bump)]
    pub vault: Account<'info, Vault>,
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