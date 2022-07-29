use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, Burn, TokenAccount};

use crate::instructions::{create_token_account::Vault, initialize::State};

use marinade_0_24_2::cpi;



pub fn order_unstake(ctx: Context<OrderUnstake>) -> Result<()> {
    let amount = ctx.accounts.burn_msol_from.amount;

    let cpi_ctx = ctx.accounts.into_marinade_order_unstake_cpi_ctx();
    cpi::order_unstake(cpi_ctx, amount)?;
    token::burn(ctx.accounts.into_spl_token_cpi_ctx(), amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct OrderUnstake<'info> {
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

    
    #[account(mut)]
    pub m_state: AccountInfo<'info>,
    #[account(mut)]
    pub msol_mint: AccountInfo<'info>,
    // Note: Ticket beneficiary is burn_msol_from.owner
    #[account(
        mut,
        token::mint = msol_mint,
        token::authority = vault
    )]
    pub burn_msol_from: Account<'info, TokenAccount>,
    // #[account(signer)]      I think the Vault PDA is required here
    // pub burn_msol_authority: AccountInfo<'info>, // burn_msol_from acc must be pre-delegated with enough amount to this key or input owner signature here
    #[account(zero, rent_exempt = enforce)]
    pub new_ticket_account: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: AccountInfo<'info>
}


impl<'info> OrderUnstake<'info> {
    pub fn into_spl_token_cpi_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Burn { 
                mint: self.mint.to_account_info(),
                from: self.token.to_account_info(),
                authority: self.authority.to_account_info(),
             }
        )
    }

    pub fn into_marinade_order_unstake_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::OrderUnstake<'info>> {
        let cpi_accounts = cpi::accounts::OrderUnstake {
            state: self.m_state.clone(),
            msol_mint: self.msol_mint.clone(),
            burn_msol_from: self.burn_msol_from.to_account_info(),
            burn_msol_authority: self.vault.to_account_info(),
            new_ticket_account: self.new_ticket_account.clone(),
            clock: self.clock.to_account_info(),
            rent: self.rent.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        CpiContext::new(self.marinade_finance_program.clone(), cpi_accounts)
    }
}
