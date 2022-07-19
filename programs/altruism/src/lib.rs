pub mod instructions;

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

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::mint_tokens::mint_tokens(ctx, amount)
    }
}
