use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, Burn, TokenAccount};

use crate::instructions::initialize::State;
use crate::state::beneficiary::Beneficiary;
use crate::state::msol_state::MsolState;

use marinade_0_24_2::cpi;

pub fn order_unstake(ctx: Context<OrderUnstake>, unstake_amount: u64) -> Result<()> {
    let share_of_msol = (unstake_amount as f64 * ctx.accounts.state.avg_entry_price) as u64;

    let cpi_ctx = ctx.accounts.into_marinade_order_unstake_cpi_ctx();
    cpi::order_unstake(cpi_ctx, share_of_msol)?;
    token::burn(ctx.accounts.into_spl_token_cpi_ctx(), unstake_amount)?;

    let lamps =  ctx.accounts.m_state.calc_lamports_from_msol_amount(share_of_msol);

    ctx.accounts.beneficiary.sol_amount += lamps;

    Ok(())
}

#[derive(Accounts)]
pub struct OrderUnstake<'info> {
    pub state: Box<Account<'info, State>>,
    #[account(mut, has_one = mint)]
    pub token: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = state.alt_sol_mint_pubkey)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut, seeds=[b"vault"], bump)]
    pub vault: AccountInfo<'info>,
    #[account(mut, seeds=[b"beneficiary", authority.key().as_ref()], bump)]
    pub beneficiary: Box<Account<'info, Beneficiary>>,

    
    #[account(mut)]
    pub m_state: Box<Account<'info, marinade_0_24_2::State>>,
    #[account(mut, address = m_state.msol_mint)]
    pub msol_mint: AccountInfo<'info>,
    // Note: Ticket beneficiary is burn_msol_from.owner
    #[account(
        mut,
        token::mint = msol_mint,
        token::authority = vault
    )]
    pub burn_msol_from: Box<Account<'info, TokenAccount>>,
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
                authority: self.vault.to_account_info(),
             }
        )
    }

    pub fn into_marinade_order_unstake_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::OrderUnstake<'info>> {
        let cpi_accounts = cpi::accounts::OrderUnstake {
            state: self.m_state.to_account_info(),
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
