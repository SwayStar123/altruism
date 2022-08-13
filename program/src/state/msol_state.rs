use marinade_0_24_2::State;

pub trait MsolState {
    fn total_cooling_down(&self) -> u64;
    fn total_lamports_under_control(&self) -> u64;
    fn total_virtual_staked_lamports(&self) -> u64;
    fn calc_msol_from_lamports(&self, stake_lamports: u64) -> u64;
    fn calc_lamports_from_msol_amount(&self, msol_amount: u64) -> u64;
}

impl MsolState for State {
    fn total_cooling_down(&self) -> u64 {
        self.stake_system
            .delayed_unstake_cooling_down
            .checked_add(self.emergency_cooling_down)
            .expect("Total cooling down overflow")
    }

    fn total_lamports_under_control(&self) -> u64 {
        self.validator_system
            .total_active_balance
            .checked_add(self.total_cooling_down())
            .expect("Stake balance overflow")
            .checked_add(self.available_reserve_balance) // reserve_pda.lamports() - self.rent_exempt_for_token_acc
            .expect("Total SOLs under control overflow")
    }

    fn total_virtual_staked_lamports(&self) -> u64 {
        // if we get slashed it may be negative but we must use 0 instead
        self.total_lamports_under_control()
            .saturating_sub(self.circulating_ticket_balance) //tickets created -> cooling down lamports or lamports already in reserve and not claimed yet
    }

    /// calculate the amount of msol tokens corresponding to certain lamport amount
    fn calc_msol_from_lamports(&self, stake_lamports: u64) -> u64 {
        shares_from_value(
            stake_lamports,
            self.total_virtual_staked_lamports(),
            self.msol_supply,
        )
    }

    fn calc_lamports_from_msol_amount(&self, msol_amount: u64) -> u64 {
        value_from_shares(
            msol_amount,
            self.total_virtual_staked_lamports(),
            self.msol_supply,
        )
    }
}

pub fn proportional(amount: u64, numerator: u64, denominator: u64) -> u64 {
    if denominator == 0 {
        return amount;
    }
    u64::try_from((amount as u128) * (numerator as u128) / (denominator as u128))
        .expect("value_from_shares calc failed")
}

#[inline] //alias for proportional
pub fn value_from_shares(shares: u64, total_value: u64, total_shares: u64) -> u64 {
    proportional(shares, total_value, total_shares)
}

pub fn shares_from_value(value: u64, total_value: u64, total_shares: u64) -> u64 {
    if total_shares == 0 {
        //no shares minted yet / First mint
        value
    } else {
        proportional(value, total_shares, total_value)
    }
}
