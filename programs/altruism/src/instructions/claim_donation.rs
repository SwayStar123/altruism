use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::instructions::create_token_account::Vault;

use marinade_0_24_2::cpi;


pub fn claim_donation(ctx: Context<ClaimDonation>) -> Result<()> {
    assert!(ctx.accounts.vault.withdrawing == false, "Vault owner is currently withdrawing, please try again later");
    let cpi_ctx = ctx.accounts.into_marinade_claim_cpi_ctx();
    cpi::claim(cpi_ctx)?;

    let empty_acc_rent = ctx.accounts.rent.minimum_balance(0);
    let sol_vault_bal = ctx.accounts.sol_vault.lamports() - empty_acc_rent;

    let donation_amount = sol_vault_bal * 995 / 1000;
    let bounty_amount = sol_vault_bal - donation_amount;

    // Bounty
    let transfer_ix = system_instruction::transfer(
        ctx.accounts.sol_vault.key, 
        ctx.accounts.authority.key, 
        bounty_amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.sol_vault.clone(),
            ctx.accounts.authority.to_account_info()
        ]
    )?;

    // Donation
    let transfer_ix = system_instruction::transfer(
        ctx.accounts.sol_vault.key, 
        ctx.accounts.global_sol_vault.key, 
        donation_amount
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.sol_vault.clone(),
            ctx.accounts.global_sol_vault.clone()
        ]
    )?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimDonation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub vault_owner: AccountInfo<'info>,
    #[account(
        mut, 
        seeds=[b"vault", vault_owner.key().as_ref()], 
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        seeds = [b"global_sol_vault"],
        bump
    )]
    pub global_sol_vault: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,

    
    #[account(mut)]
    pub m_state: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut)]
    pub ticket_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"sol_vault", vault_owner.key().as_ref()],
        bump
    )]
    pub sol_vault: AccountInfo<'info>,
    
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: AccountInfo<'info>
}


impl<'info> ClaimDonation<'info> {
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
