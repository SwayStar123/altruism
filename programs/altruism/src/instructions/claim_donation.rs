use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::state::beneficiary::Beneficiary;

use marinade_0_24_2::cpi;


pub fn claim_donation(ctx: Context<ClaimDonation>) -> Result<()> {

    let cpi_ctx = ctx.accounts.into_marinade_claim_cpi_ctx();
    cpi::claim(cpi_ctx)?;

    let beneficiary_amount = ctx.accounts.beneficiary.sol_amount;

    let donation_amount = beneficiary_amount * 995 / 1000;
    let bounty_amount = beneficiary_amount - donation_amount;

    // Bounty
    let transfer_ix = system_instruction::transfer(
        ctx.accounts.vault.key, 
        ctx.accounts.authority.key, 
        bounty_amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.vault.clone(),
            ctx.accounts.authority.to_account_info()
        ]
    )?;

    // Donation
    let transfer_ix = system_instruction::transfer(
        ctx.accounts.vault.key, 
        ctx.accounts.global_sol_vault.key, 
        donation_amount
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.vault.clone(),
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
        seeds=[b"msol_vault"], 
        bump
    )]
    pub vault: AccountInfo<'info>,
    #[account(
        seeds = [b"global_sol_vault"],
        bump
    )]
    pub global_sol_vault: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"beneficiary"],
        bump,
    )]
    pub beneficiary: Account<'info, Beneficiary>,
    pub rent: Sysvar<'info, Rent>,

    
    #[account(mut)]
    pub m_state: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut)]
    pub ticket_account: AccountInfo<'info>,

    
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
            transfer_sol_to: self.vault.clone(),
            clock: self.clock.to_account_info(),
            system_program: self.system_program.to_account_info(),
           
        };

        CpiContext::new(self.marinade_finance_program.clone(), cpi_accounts)
    }
}
