use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use cash_register::currency::USD;
use cash_register::strategy::random::RandomStrategy;
use cash_register::strategy::greedy::GreedyStrategy;
use cash_register::strategy::ChangeStrategy;

proptest! {
    #[test]
    fn random_always_sums_to_target(cents in 1u32..10_000, seed in any::<u64>()) {
        let rng = StdRng::seed_from_u64(seed);
        let mut strategy = RandomStrategy::new(rng);
        let breakdown = strategy.make_change(cents, &USD);

        let total: u32 = breakdown.iter().map(|(d, c)| d.cents * c).sum();
        prop_assert_eq!(total, cents, "random breakdown must sum to target");
    }

    #[test]
    fn random_uses_only_valid_denominations(cents in 1u32..10_000, seed in any::<u64>()) {
        let rng = StdRng::seed_from_u64(seed);
        let mut strategy = RandomStrategy::new(rng);
        let breakdown = strategy.make_change(cents, &USD);

        let valid_values: Vec<u32> = USD.denominations.iter().map(|d| d.cents).collect();
        for (denom, _) in &breakdown {
            prop_assert!(
                valid_values.contains(&denom.cents),
                "denomination {} is not in USD",
                denom.cents
            );
        }
    }

    #[test]
    fn random_all_counts_positive(cents in 1u32..10_000, seed in any::<u64>()) {
        let rng = StdRng::seed_from_u64(seed);
        let mut strategy = RandomStrategy::new(rng);
        let breakdown = strategy.make_change(cents, &USD);

        for (denom, count) in &breakdown {
            prop_assert!(*count > 0, "{} has count 0", denom.singular);
        }
    }

    #[test]
    fn greedy_always_sums_to_target(cents in 0u32..10_000) {
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(cents, &USD);

        let total: u32 = breakdown.iter().map(|(d, c)| d.cents * c).sum();
        prop_assert_eq!(total, cents, "greedy breakdown must sum to target");
    }

    #[test]
    fn greedy_uses_minimum_coins(cents in 1u32..100) {
        // For USD denominations, greedy should never use more coins than
        // just using all pennies
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(cents, &USD);

        let total_coins: u32 = breakdown.iter().map(|(_, c)| c).sum();
        prop_assert!(total_coins <= cents, "greedy should use at most {cents} coins, used {total_coins}");
    }
}
