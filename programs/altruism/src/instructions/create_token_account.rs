use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_spl::token::{Token, Mint, TokenAccount};

pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.bump = *ctx.bumps.get("vault").unwrap();

    let sol_vault_rent_exemption_amt = ctx.accounts.rent.minimum_balance(0);
    let transfer_ix = system_instruction::transfer(
        ctx.accounts.authority.key,
        ctx.accounts.sol_vault.key, 
        sol_vault_rent_exemption_amt);

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            //from
            ctx.accounts.authority.to_account_info(),
            //to
            ctx.accounts.sol_vault.to_account_info()
        ]
    )?;
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
    #[account(
        seeds = [b"sol_vault", authority.key().as_ref()],
        bump,
    )]
    pub sol_vault: AccountInfo<'info>,
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