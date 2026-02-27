use crate::currency::Currency;
use super::{Breakdown, ChangeStrategy};

/// Greedy algorithm: use the fewest coins/bills possible.
///
/// Iterates denominations largest-to-smallest, taking as many of each as
/// possible before moving to the next. Produces minimum denomination count
/// when the denomination set has the greedy property (true for USD and EUR).
pub struct GreedyStrategy;

impl ChangeStrategy for GreedyStrategy {
    fn make_change(&mut self, mut cents: u32, currency: &Currency) -> Breakdown {
        let mut result = Vec::new();

        for &denom in currency.denominations {
            if cents == 0 {
                break;
            }
            let count = cents / denom.cents;
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

    #[test]
    fn sample_output_88_cents() {
        // 3.00 - 2.12 = 0.88 -> 3 quarters, 1 dime, 3 pennies
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(88, &USD);

        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(
            named,
            vec![("quarter", 3), ("dime", 1), ("penny", 3)],
        );
    }

    #[test]
    fn sample_output_3_cents() {
        // 2.00 - 1.97 = 0.03 -> 3 pennies
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(3, &USD);

        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(named, vec![("penny", 3)]);
    }

    #[test]
    fn exact_dollar_amount() {
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(300, &USD);

        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(named, vec![("dollar", 3)]);
    }

    #[test]
    fn zero_change() {
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(0, &USD);
        assert!(breakdown.is_empty());
    }

    #[test]
    fn uses_all_denominations() {
        // 141 = 100 + 25 + 10 + 5 + 1
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(141, &USD);

        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(
            named,
            vec![
                ("dollar", 1),
                ("quarter", 1),
                ("dime", 1),
                ("nickel", 1),
                ("penny", 1),
            ],
        );
    }

    #[test]
    fn single_penny() {
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(1, &USD);
        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(named, vec![("penny", 1)]);
    }

    #[test]
    fn large_amount() {
        // $99.99 = 9999 cents
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(9999, &USD);
        let total: u32 = breakdown.iter().map(|(d, c)| d.cents * c).sum();
        assert_eq!(total, 9999);
        // Should be 99 dollars, 3 quarters, 2 dimes, 4 pennies
        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(named, vec![("dollar", 99), ("quarter", 3), ("dime", 2), ("penny", 4)]);
    }

    #[test]
    fn eur_greedy_63_cents() {
        // 63 = 50 + 10 + 2 + 1
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(63, &EUR);

        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(
            named,
            vec![
                ("50 cent coin", 1),
                ("10 cent coin", 1),
                ("2 cent coin", 1),
                ("1 cent coin", 1),
            ],
        );
    }

    #[test]
    fn eur_greedy_387_cents() {
        // 387 = 200 + 100 + 50 + 20 + 10 + 5 + 2
        let mut strategy = GreedyStrategy;
        let breakdown = strategy.make_change(387, &EUR);

        let named: Vec<(&str, u32)> = breakdown.iter().map(|(d, c)| (d.singular, *c)).collect();
        assert_eq!(
            named,
            vec![
                ("2 euro coin", 1),
                ("1 euro coin", 1),
                ("50 cent coin", 1),
                ("20 cent coin", 1),
                ("10 cent coin", 1),
                ("5 cent coin", 1),
                ("2 cent coin", 1),
            ],
        );
    }
}
