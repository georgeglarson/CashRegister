use rand::Rng;

use crate::currency::Currency;
use crate::parse::Transaction;
use crate::strategy::greedy::GreedyStrategy;
use crate::strategy::random::RandomStrategy;
use crate::strategy::{Breakdown, ChangeStrategy};

/// Determine change for a transaction, dispatching to the appropriate strategy.
///
/// If `owed_cents` is divisible by `divisor`, uses randomized denominations.
/// Otherwise, uses the greedy (minimum count) algorithm.
pub fn make_change_for<R: Rng>(
    transaction: &Transaction,
    currency: &Currency,
    divisor: u32,
    rng: &mut R,
) -> Breakdown {
    if transaction.change_cents == 0 {
        return Vec::new();
    }

    if divisor > 0 && transaction.owed_cents.is_multiple_of(divisor) {
        RandomStrategy::new(rng).make_change(transaction.change_cents, currency)
    } else {
        GreedyStrategy.make_change(transaction.change_cents, currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::USD;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn tx(owed: u32, paid: u32) -> Transaction {
        Transaction {
            owed_cents: owed,
            paid_cents: paid,
            change_cents: paid - owed,
        }
    }

    #[test]
    fn divisible_by_3_uses_random() {
        let mut rng = StdRng::seed_from_u64(42);

        // 333 is divisible by 3
        let random_result = make_change_for(&tx(333, 500), &USD, 3, &mut rng);
        let total: u32 = random_result.iter().map(|(d, c)| d.cents * c).sum();
        assert_eq!(total, 167);
    }

    #[test]
    fn not_divisible_by_3_uses_greedy() {
        let mut rng = StdRng::seed_from_u64(42);

        // 212 is not divisible by 3
        let result = make_change_for(&tx(212, 300), &USD, 3, &mut rng);
        let named: Vec<(&str, u32)> = result.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(named, vec![("quarter", 3), ("dime", 1), ("penny", 3)]);
    }

    #[test]
    fn zero_divisor_always_uses_greedy() {
        let mut rng = StdRng::seed_from_u64(42);

        // Even though 300 is divisible by 3, divisor is 0 so greedy is used
        let result = make_change_for(&tx(300, 500), &USD, 0, &mut rng);
        let named: Vec<(&str, u32)> = result.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(named, vec![("dollar", 2)]);
    }

    #[test]
    fn custom_divisor() {
        let mut rng = StdRng::seed_from_u64(42);

        // 500 is divisible by 5 -> random
        let result = make_change_for(&tx(500, 700), &USD, 5, &mut rng);
        let total: u32 = result.iter().map(|(d, c)| d.cents * c).sum();
        assert_eq!(total, 200);
    }

    #[test]
    fn exact_payment_returns_empty() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = make_change_for(&tx(300, 300), &USD, 3, &mut rng);
        assert!(result.is_empty());
    }
}
