pub mod greedy;
pub mod random;

use crate::currency::{Currency, Denomination};

/// A breakdown of change: pairs of (denomination, count).
/// Only includes denominations with count > 0.
pub type Breakdown = Vec<(Denomination, u32)>;

/// A strategy for making change.
pub trait ChangeStrategy {
    fn make_change(&mut self, cents: u32, currency: &Currency) -> Breakdown;
}
