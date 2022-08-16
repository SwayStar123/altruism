use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use crate::state::*;

use marinade_0_24_2::cpi;

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_ctx = ctx.accounts.into_marinade_deposit_cpi_ctx();

    cpi::deposit(cpi_ctx, amount)?;
    token::mint_to(ctx.accounts.into_spl_token_cpi_ctx(), amount)?;

    ctx.accounts.state.total_deposited += amount;
    ctx.accounts.state.avg_entry_price = ((ctx.accounts.state.total_deposited as u128 * 1000000
        / ctx.accounts.mint_to.amount as u128
        * 1000000)
        / 1000000) as u64;
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub state: Box<Account<'info, AltruismState>>,
    #[account(mut, has_one = mint)]
    pub token: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = state.alt_sol_mint_pubkey)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: trust me bro
    #[account(mut, seeds=[b"msol_vault".as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,

    // this part is equivalent to marinade-finance deposit instructions
    #[account(mut)]
    pub marinade_state: Box<Account<'info, MarinadeState>>, // marinade state 8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC
    #[account(mut, address = marinade_state.msol_mint)]
    pub msol_mint: Box<Account<'info, Mint>>, // mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So
    /// CHECK: trust me bro
    #[account(mut)]
    pub liq_pool_sol_leg_pda: UncheckedAccount<'info>, // UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q
    /// CHECK: trust me bro
    #[account(mut)]
    pub liq_pool_msol_leg: UncheckedAccount<'info>, // 7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE
    /// CHECK: trust me bro
    pub liq_pool_msol_leg_authority: UncheckedAccount<'info>, // EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL
    /// CHECK: trust me bro
    #[account(mut)]
    pub reserve_pda: UncheckedAccount<'info>, // Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN
    #[account(
        mut,
        token::mint = msol_mint,
        token::authority = vault,
    )]
    pub mint_to: Box<Account<'info, TokenAccount>>,
    /// CHECK: trust me bro
    pub msol_mint_authority: UncheckedAccount<'info>, // 3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: trust me bro
    pub system_program: UncheckedAccount<'info>,
    /// CHECK: trust me bro
    #[account(address = marinade_0_24_2::ID)]
    pub marinade_finance_program: UncheckedAccount<'info>, // MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD
}

impl<'info> Deposit<'info> {
    pub fn into_spl_token_cpi_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }

    pub fn into_marinade_deposit_cpi_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::Deposit<'info>> {
        let cpi_accounts = cpi::accounts::Deposit {
            state: self.marinade_state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: self.liq_pool_msol_leg_authority.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            transfer_from: self.authority.to_account_info(),
            mint_to: self.mint_to.to_account_info(),
            msol_mint_authority: self.msol_mint_authority.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        CpiContext::new(
            self.marinade_finance_program.to_account_info(),
            cpi_accounts,
        )
    }
}
