use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, Burn, TokenAccount};

use crate::instructions::{create_token_account::Vault, initialize::State};

use marinade_0_24_2::cpi;



pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    

    let cpi_ctx = ctx.accounts.into_marinade_withdraw_cpi_ctx();

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
    pub marinade_state: AccountInfo<'info>,
    // Readonly. For stake delta calculation
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut)]
    pub validator_list: AccountInfo<'info>,
    #[account(mut)]
    pub stake_list: AccountInfo<'info>,
    #[account(mut)]
    pub stake_account: AccountInfo<'info>,
    pub stake_deposit_authority: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub epoch_schedule: Sysvar<'info, EpochSchedule>,
    pub stake_history: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub stake_program: AccountInfo<'info>,
   
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: AccountInfo<'info>
}


impl<'info> Withdraw<'info> {
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

    pub fn into_marinade_withdraw_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::DeactivateStake<'info>> {
        let cpi_accounts = cpi::accounts::DeactivateStake {
            state: self.marinade_state.clone(),
            reserve_pda: self.reserve_pda.clone(),
            validator_list: self.validator_list.clone(),
            stake_list: self.stake_list.clone(),
            stake_account: self.stake_account.clone(),
            stake_deposit_authority: self.stake_deposit_authority.clone(),
            split_stake_account: self.authority.to_account_info(),
            split_stake_rent_payer: self.authority.to_account_info(),
            clock: self.clock.to_account_info(),
            rent: self.rent.to_account_info(),
            epoch_schedule: self.epoch_schedule.to_account_info(),
            stake_history: self.stake_history.clone(),
            system_program: self.system_program.to_account_info(),
            stake_program: self.stake_program.clone(),
        };

        CpiContext::new(self.marinade_finance_program.clone(), cpi_accounts)
    }
}
