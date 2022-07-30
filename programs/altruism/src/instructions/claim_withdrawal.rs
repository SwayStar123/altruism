use std::iter::empty;

use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_spl::token::{Token, Mint, TokenAccount};

use crate::instructions::{create_token_account::Vault, initialize::State};

use marinade_0_24_2::cpi;


pub fn claim_withdrawal(ctx: Context<ClaimWithdrawal>) -> Result<()> {
    let cpi_ctx = ctx.accounts.into_marinade_claim_cpi_ctx();
    cpi::claim(cpi_ctx)?;

    let empty_acc_rent = ctx.accounts.rent.minimum_balance(0);
    let deposited = ctx.accounts.vault.deposited;
    let sol_vault_bal = ctx.accounts.sol_vault.lamports() - empty_acc_rent;

    // If the profit is negative due to slashing, just give all to depositer,
    // if profit is made, donate the excess
    let deserved_amount = if deposited >= sol_vault_bal {
        sol_vault_bal
    } else {
        deposited
    };

    let transfer_ix = system_instruction::transfer(
        ctx.accounts.sol_vault.key, 
        ctx.accounts.authority.key, 
        deserved_amount
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.sol_vault.clone(),
            ctx.accounts.authority.to_account_info()
        ]
    )?;

    ctx.accounts.vault.deposited = 0;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimWithdrawal<'info> {
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
    pub rent: Sysvar<'info, Rent>,
    
    #[account(mut)]
    pub m_state: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut)]
    pub ticket_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"sol_vault", authority.key().as_ref()],
        bump
    )]
    pub sol_vault: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: AccountInfo<'info>
}


impl<'info> ClaimWithdrawal<'info> {
    pub fn into_marinade_claim_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::Claim<'info>> {
        let cpi_accounts = cpi::accounts::Claim {
            state: self.m_state.clone(),
            reserve_pda: self.reserve_pda.clone(),
            ticket_account: self.ticket_account.clone(),
            transfer_sol_to: self.sol_vault.clone(),
            clock: self.clock.to_account_info(),
            system_program: self.system_program.to_account_info(),
           
        };

        CpiContext::new(self.marinade_finance_program.clone(), cpi_accounts)
    }
}
