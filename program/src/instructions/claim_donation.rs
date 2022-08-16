use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::state::*;

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
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ],
    )?;

    // Donation
    let transfer_ix = system_instruction::transfer(
        ctx.accounts.vault.key,
        ctx.accounts.global_sol_vault.key,
        donation_amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.global_sol_vault.to_account_info(),
        ],
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimDonation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: trust me bro
    #[account(
        mut,
        seeds=[b"msol_vault".as_ref()], 
        bump
    )]
    pub vault: UncheckedAccount<'info>,
    /// CHECK: trust me bro
    #[account(
        seeds = [b"global_sol_vault".as_ref()],
        bump
    )]
    pub global_sol_vault: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"beneficiary".as_ref()],
        bump,
    )]
    pub beneficiary: Account<'info, Beneficiary>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: trust me bro
    #[account(mut)]
    pub m_state: UncheckedAccount<'info>,
    /// CHECK: trust me bro
    #[account(mut)]
    pub reserve_pda: UncheckedAccount<'info>,
    /// CHECK: trust me bro
    #[account(mut)]
    pub ticket_account: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    /// CHECK: trust me bro
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: UncheckedAccount<'info>,
}

impl<'info> ClaimDonation<'info> {
    pub fn into_marinade_claim_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::Claim<'info>> {
        let cpi_accounts = cpi::accounts::Claim {
            state: self.m_state.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            ticket_account: self.ticket_account.to_account_info(),
            transfer_sol_to: self.vault.to_account_info(),
            clock: self.clock.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(
            self.marinade_finance_program.to_account_info(),
            cpi_accounts,
        )
    }
}
