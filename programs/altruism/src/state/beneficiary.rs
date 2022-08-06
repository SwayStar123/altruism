use anchor_lang::prelude::*;

#[account]
pub struct Beneficiary { // 8
    pub sol_amount: u64, // 8
    // pub beneficiary: Pubkey, // 32
}