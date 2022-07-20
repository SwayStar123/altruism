use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.bump = *ctx.bumps.get("vault").unwrap();
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
    #[account(
        init,
        payer = authority,
        space = 41 + 8,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

// 32 + 8 + 1 + 8
#[account]
pub struct Vault {
    pub authority: Pubkey, //32
    pub deposited: u64, // 8
    pub bump: u8, // 1
}