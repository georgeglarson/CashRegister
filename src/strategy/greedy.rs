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
    use crate::currency::USD;

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
}
