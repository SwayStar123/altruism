use anchor_lang::prelude::*;
pub mod msol_state;
pub use msol_state::*;

#[account]
pub struct Beneficiary {
    // 8
    pub sol_amount: u64, // 8
                         // pub beneficiary: Pubkey, // 32
}

// 32
#[account]
pub struct AltruismState {
    pub alt_sol_mint_pubkey: Pubkey, // 32
    pub total_deposited: u64,        // 8
    pub avg_entry_price: u64,        // 8
}
