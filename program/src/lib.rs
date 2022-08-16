pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("A1tru1e86JujDZ1jP2yaoCHegVj8SvMzSP2yhRFDQgTR");

#[program]
pub mod altruism_finance {
    use super::*;

    // Called at the start to setup all the necessary program accounts
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    // function for anyone to make a altSOL token account
    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        instructions::create_token_account::create_token_account(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    // function for anyone to deposit SOL and get altSOL (backed by mSOL)
    pub fn mint_tokens(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    // orders a delayed unstake of the mSOL and consecuticely the altSOL
    pub fn order_unstake(ctx: Context<OrderUnstake>, amount: u64) -> Result<()> {
        instructions::order_unstake::order_unstake(ctx, amount)
    }

    // claims the delayed unstake ticket
    pub fn claim_withdrawal(ctx: Context<ClaimWithdrawal>) -> Result<()> {
        instructions::claim_withdrawal::claim_withdrawal(ctx)
    }

    // allows ANYONE to order an unstake of the amount over the deposit amount
    pub fn unstake_donation(ctx: Context<UnstakeDonation>) -> Result<()> {
        instructions::unstake_donation::unstake_donation(ctx)
    }

    // allows ANYONE to claim the excess unstaked amount and donate it
    pub fn claim_donation(ctx: Context<ClaimDonation>) -> Result<()> {
        instructions::claim_donation::claim_donation(ctx)
    }
}
