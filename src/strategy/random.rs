use rand::Rng;

use crate::currency::Currency;
use super::{Breakdown, ChangeStrategy};

/// Randomized change algorithm: pick random counts for each denomination.
///
/// For each denomination (largest to smallest), picks a random count between
/// 0 and the maximum possible. The final denomination (1 cent) absorbs the
/// remainder, guaranteeing the total is always exact.
///
/// Generic over `R: Rng` so tests can inject a seeded RNG for determinism.
pub struct RandomStrategy<R: Rng> {
    rng: R,
}

impl<R: Rng> RandomStrategy<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> ChangeStrategy for RandomStrategy<R> {
    fn make_change(&mut self, mut cents: u32, currency: &Currency) -> Breakdown {
        let mut result = Vec::new();
        let denoms = currency.denominations;

        for (i, &denom) in denoms.iter().enumerate() {
            if cents == 0 {
                break;
            }

            let max_count = cents / denom.cents;
            if max_count == 0 {
                continue;
            }

            let is_last = i == denoms.len() - 1;
            let count = if is_last {
                // Last denomination absorbs the remainder — guarantees exact change
                max_count
            } else {
                self.rng.gen_range(0..=max_count)
            };

            if count > 0 {
                result.push((denom, count));
                cents -= count * denom.cents;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::{USD, EUR};
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn seeded_strategy(seed: u64) -> RandomStrategy<StdRng> {
        RandomStrategy::new(StdRng::seed_from_u64(seed))
    }

    #[test]
    fn random_change_sums_correctly() {
        for seed in 0..100 {
            let mut strategy = seeded_strategy(seed);
            let target = 167u32; // $1.67

            let breakdown = strategy.make_change(target, &USD);
            let total: u32 = breakdown.iter().map(|(d, c)| d.cents * c).sum();

            assert_eq!(
                total, target,
                "seed {seed}: breakdown sums to {total}, expected {target}"
            );
        }
    }

    #[test]
    fn random_change_all_counts_positive() {
        for seed in 0..100 {
            let mut strategy = seeded_strategy(seed);
            let breakdown = strategy.make_change(250, &USD);

            for (denom, count) in &breakdown {
                assert!(
                    *count > 0,
                    "seed {seed}: {} has count 0 but appeared in breakdown",
                    denom.singular
                );
            }
        }
    }

    #[test]
    fn random_change_zero() {
        let mut strategy = seeded_strategy(42);
        let breakdown = strategy.make_change(0, &USD);
        assert!(breakdown.is_empty());
    }

    #[test]
    fn deterministic_with_same_seed() {
        let b1 = seeded_strategy(42).make_change(167, &USD);
        let b2 = seeded_strategy(42).make_change(167, &USD);

        let counts1: Vec<u32> = b1.iter().map(|(_, c)| *c).collect();
        let counts2: Vec<u32> = b2.iter().map(|(_, c)| *c).collect();
        assert_eq!(counts1, counts2);
    }

    #[test]
    fn different_seeds_can_produce_different_results() {
        // Not guaranteed, but with enough seeds at least one should differ
        let baseline = seeded_strategy(0).make_change(500, &USD);
        let baseline_counts: Vec<u32> = baseline.iter().map(|(_, c)| *c).collect();

        let any_different = (1..50).any(|seed| {
            let b = seeded_strategy(seed).make_change(500, &USD);
            let counts: Vec<u32> = b.iter().map(|(_, c)| *c).collect();
            counts != baseline_counts
        });

        assert!(any_different, "random strategy should produce varied results across seeds");
    }

    #[test]
    fn eur_random_sums_correctly() {
        for seed in 0..100 {
            let mut strategy = seeded_strategy(seed);
            let target = 263u32; // EUR has 2-cent coins and 20-cent coins — different structure
            let breakdown = strategy.make_change(target, &EUR);
            let total: u32 = breakdown.iter().map(|(d, c)| d.cents * c).sum();
            assert_eq!(
                total, target,
                "seed {seed}: EUR breakdown sums to {total}, expected {target}"
            );
        }
    }
}
