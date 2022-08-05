pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("3QRDFYBJULeGi75Qon1HpDDRMuN1zqwNU9DQ1sGSSQYc");

#[program]
pub mod altruism {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        instructions::create_token_account::create_token_account(ctx)
    }

    pub fn mint_tokens(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    pub fn order_unstake(ctx: Context<OrderUnstake>) -> Result<()> {
        instructions::order_unstake::order_unstake(ctx)
    }

    pub fn claim_withdrawal(ctx: Context<ClaimWithdrawal>) -> Result<()> {
        instructions::claim_withdrawal::claim_withdrawal(ctx)
    }

    pub fn unstake_donation(ctx: Context<UnstakeDonation>) -> Result<()> {
        instructions::unstake_donation::unstake_donation(ctx)
    }

    pub fn claim_donation(ctx: Context<ClaimDonation>) -> Result<()> {
        instructions::claim_donation::claim_donation(ctx)
    }    
}
