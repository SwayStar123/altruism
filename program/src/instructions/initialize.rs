use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.state.alt_sol_mint_pubkey = ctx.accounts.mint.key();

    let empty_acc_rent = ctx.accounts.rent.minimum_balance(0);

    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        ctx.accounts.payer.key,
        ctx.accounts.global_sol_vault.key,
        empty_acc_rent,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            //from
            ctx.accounts.payer.to_account_info(),
            //to
            ctx.accounts.global_sol_vault.to_account_info(),
        ],
    )?;

    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        ctx.accounts.payer.key,
        ctx.accounts.vault.key,
        empty_acc_rent,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            //from
            ctx.accounts.payer.to_account_info(),
            //to
            ctx.accounts.vault.to_account_info(),
        ],
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
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 48 + 8,
    )]
    pub state: Box<Account<'info, AltruismState>>,
    /// CHECK: trust me bro
    #[account(
        mut,
        seeds = [b"global_sol_vault".as_ref()],
        bump
    )]
    pub global_sol_vault: UncheckedAccount<'info>,
    /// CHECK: trust me bro
    #[account(mut, seeds=[b"msol_vault".as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub m_state: Box<Account<'info, MarinadeState>>,
    /// CHECK: trust me bro
    #[account(mut, address = m_state.msol_mint)]
    pub msol_mint: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        token::mint = msol_mint,
        token::authority = vault
    )]
    pub msol_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8,
        seeds = [b"beneficiary".as_ref()],
        bump,
    )]
    pub beneficiary: Box<Account<'info, Beneficiary>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
