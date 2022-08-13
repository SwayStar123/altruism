use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::instructions::initialize;
use crate::state::{beneficiary::Beneficiary, msol_state::MsolState};

use marinade_0_24_2::cpi;

pub fn unstake_donation(ctx: Context<UnstakeDonation>) -> Result<()> {
    let total_msol = ctx.accounts.burn_msol_from.amount;
    let total_altsol = ctx.accounts.mint.supply;
    let current_price = ctx
        .accounts
        .m_state
        .calc_lamports_from_msol_amount(total_msol) as f64
        / total_altsol as f64;

    assert!(1.0 < current_price);
    let price_diff = 1.0 - current_price;

    let excess_msol = (price_diff * total_msol as f64) as u64;

    ctx.accounts.beneficiary.sol_amount += ctx
        .accounts
        .m_state
        .calc_lamports_from_msol_amount(excess_msol);

    let cpi_ctx = ctx.accounts.into_marinade_order_unstake_cpi_ctx();
    cpi::order_unstake(cpi_ctx, excess_msol)?;

    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeDonation<'info> {
    pub state: Box<Account<'info, initialize::State>>,
    #[account(mut, has_one = mint)]
    pub token: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = state.alt_sol_mint_pubkey)]
    pub mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    #[account(mut, seeds=[b"msol_vault"], bump)]
    /// CHECK: trust me bro
    pub vault: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"beneficiary"],
        bump,
    )]
    pub beneficiary: Box<Account<'info, Beneficiary>>,

    #[account(mut)]
    pub m_state: Box<Account<'info, marinade_0_24_2::State>>,
    /// CHECK: trust me bro
    #[account(mut)]
    pub msol_mint: UncheckedAccount<'info>,
    // Note: Ticket beneficiary is burn_msol_from.owner
    #[account(
        mut,
        token::mint = msol_mint,
        token::authority = vault
    )]
    pub burn_msol_from: Box<Account<'info, TokenAccount>>,
    // #[account(signer)]      I think the Vault PDA is required here
    // pub burn_msol_authority: UncheckedAccount<'info>, // burn_msol_from acc must be pre-delegated with enough amount to this key or input owner signature here
    /// CHECK: trust me bro
    #[account(zero, rent_exempt = enforce)]
    pub new_ticket_account: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: trust me bro
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: UncheckedAccount<'info>,
}

impl<'info> UnstakeDonation<'info> {
    pub fn into_marinade_order_unstake_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::OrderUnstake<'info>> {
        let cpi_accounts = cpi::accounts::OrderUnstake {
            state: self.m_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            burn_msol_from: self.burn_msol_from.to_account_info(),
            burn_msol_authority: self.vault.to_account_info(),
            new_ticket_account: self.new_ticket_account.to_account_info(),
            clock: self.clock.to_account_info(),
            rent: self.rent.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        CpiContext::new(
            self.marinade_finance_program.to_account_info(),
            cpi_accounts,
        )
    }
}
