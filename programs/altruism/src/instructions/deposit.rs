use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, TokenAccount};

use crate::instructions::{create_token_account::Vault, initialize::State};

use marinade_0_24_2::cpi;

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {

    let cpi_ctx = ctx.accounts.into_marinade_deposit_cpi_ctx();

    cpi::deposit(cpi_ctx, amount)?;
    token::mint_to(ctx.accounts.into_spl_token_cpi_ctx(), amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub state: Account<'info, State>,
    #[account(mut, has_one = mint)]
    pub token: Account<'info, TokenAccount>,
    #[account(mut, address = state.alt_sol_mint_pubkey)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut, seeds=[b"vault", authority.key().as_ref()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    // this part is equivalent to marinade-finance deposit instructions
    #[account(mut)]
    pub marinade_state: AccountInfo<'info>, // marinade state
    #[account(mut)]
    pub msol_mint: Account<'info, Mint>,
    #[account(mut)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        token::mint = msol_mint,
        token::authority = vault,
    )]
    pub mint_to: Account<'info, TokenAccount>,
    pub msol_mint_authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
    

    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: AccountInfo<'info>
}


impl<'info> Deposit<'info> {
    pub fn into_spl_token_cpi_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo { 
                mint: self.mint.to_account_info(),
                to: self.token.to_account_info(),
                authority: self.authority.to_account_info(),
             }
        )
    }

    pub fn into_marinade_deposit_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::Deposit<'info>> {
        let cpi_accounts = cpi::accounts::Deposit {
            state: self.marinade_state.clone(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.clone(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.clone(),
            liq_pool_msol_leg_authority: self.liq_pool_msol_leg_authority.clone(),
            reserve_pda: self.reserve_pda.clone(),
            transfer_from: self.authority.to_account_info(),
            mint_to: self.mint_to.to_account_info(),
            msol_mint_authority: self.msol_mint_authority.clone(),
            system_program: self.system_program.clone(),
            token_program: self.token_program.to_account_info(),
        };

        CpiContext::new(self.marinade_finance_program.clone(), cpi_accounts)
    }
}