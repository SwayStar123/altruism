use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint};

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.state.alt_sol_mint_pubkey = ctx.accounts.mint.key();

    let global_sol_vault_rent_exemption_amt = ctx.accounts.rent.minimum_balance(0);
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        ctx.accounts.payer.key,
        ctx.accounts.global_sol_vault.key, 
        global_sol_vault_rent_exemption_amt);

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            //from
            ctx.accounts.payer.to_account_info(),
            //to
            ctx.accounts.global_sol_vault.to_account_info()
        ]
    )?;
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
    #[account(
        seeds = [b"global_sol_vault"],
        bump
    )]
    pub global_sol_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

// 32
#[account]
pub struct State {
    pub alt_sol_mint_pubkey: Pubkey, // 32
}