use crate::currency::Currency;
use crate::parse::Transaction;
use crate::strategy::Breakdown;

/// Format a breakdown into the output string.
///
/// Examples:
/// - `[(quarter, 3), (dime, 1), (penny, 3)]` -> `"3 quarters,1 dime,3 pennies"`
/// - `[]` -> `"no change"`
///
/// Uses singular/plural from the denomination and joins with commas.
pub fn format_breakdown(breakdown: &Breakdown) -> String {
    if breakdown.is_empty() {
        return "no change".to_string();
    }

    breakdown
        .iter()
        .map(|(denom, count)| {
            let name = if *count == 1 {
                denom.singular
            } else {
                denom.plural
            };
            format!("{count} {name}")
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Format cents with a currency symbol: 213, "$" -> "$2.13".
fn format_amount(cents: u32, symbol: &str) -> String {
    format!("{symbol}{}.{:02}", cents / 100, cents % 100)
}

/// Format a transaction with its breakdown for verbose output.
///
/// Example: "Owed $2.12, Paid $3.00 -> 3 quarters,1 dime,3 pennies"
/// With randomization: "Owed $3.33, Paid $5.00 -> 1 dollar,2 quarters (random)"
pub fn format_verbose(
    transaction: &Transaction,
    breakdown: &Breakdown,
    currency: &Currency,
    is_random: bool,
) -> String {
    let change = format_breakdown(breakdown);
    let label = if is_random { " (random)" } else { "" };
    let sym = currency.symbol;
    format!(
        "Owed {}, Paid {} -> {change}{label}",
        format_amount(transaction.owed_cents, sym),
        format_amount(transaction.paid_cents, sym),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Denomination;

    fn penny() -> Denomination {
        Denomination {
            cents: 1,
            singular: "penny",
            plural: "pennies",
        }
    }

    fn quarter() -> Denomination {
        Denomination {
            cents: 25,
            singular: "quarter",
            plural: "quarters",
        }
    }

    fn dime() -> Denomination {
        Denomination {
            cents: 10,
            singular: "dime",
            plural: "dimes",
        }
    }

    fn dollar() -> Denomination {
        Denomination {
            cents: 100,
            singular: "dollar",
            plural: "dollars",
        }
    }

    #[test]
    fn sample_output_format() {
        let breakdown = vec![(quarter(), 3), (dime(), 1), (penny(), 3)];
        assert_eq!(format_breakdown(&breakdown), "3 quarters,1 dime,3 pennies");
    }

    #[test]
    fn single_denomination_singular() {
        let breakdown = vec![(dollar(), 1)];
        assert_eq!(format_breakdown(&breakdown), "1 dollar");
    }

    #[test]
    fn single_denomination_plural() {
        let breakdown = vec![(penny(), 5)];
        assert_eq!(format_breakdown(&breakdown), "5 pennies");
    }

    #[test]
    fn empty_breakdown() {
        assert_eq!(format_breakdown(&Vec::new()), "no change");
    }

    #[test]
    fn matches_exact_sample_output() {
        // "3 quarters,1 dime,3 pennies" — note: no spaces after commas
        let breakdown = vec![(quarter(), 3), (dime(), 1), (penny(), 3)];
        let output = format_breakdown(&breakdown);
        assert!(
            !output.contains(", "),
            "output should not have spaces after commas"
        );
        assert_eq!(output, "3 quarters,1 dime,3 pennies");
    }

    #[test]
    fn format_amount_usd() {
        assert_eq!(format_amount(213, "$"), "$2.13");
        assert_eq!(format_amount(5, "$"), "$0.05");
        assert_eq!(format_amount(300, "$"), "$3.00");
        assert_eq!(format_amount(0, "$"), "$0.00");
        assert_eq!(format_amount(10000, "$"), "$100.00");
    }

    #[test]
    fn format_amount_eur() {
        assert_eq!(format_amount(150, "€"), "€1.50");
        assert_eq!(format_amount(1, "€"), "€0.01");
    }

    #[test]
    fn verbose_greedy() {
        let tx = Transaction {
            owed_cents: 212,
            paid_cents: 300,
            change_cents: 88,
        };
        let breakdown = vec![(quarter(), 3), (dime(), 1), (penny(), 3)];
        assert_eq!(
            format_verbose(&tx, &breakdown, &crate::currency::USD, false),
            "Owed $2.12, Paid $3.00 -> 3 quarters,1 dime,3 pennies",
        );
    }

    #[test]
    fn verbose_random() {
        let tx = Transaction {
            owed_cents: 333,
            paid_cents: 500,
            change_cents: 167,
        };
        let breakdown = vec![(dollar(), 1), (quarter(), 2), (penny(), 17)];
        assert_eq!(
            format_verbose(&tx, &breakdown, &crate::currency::USD, true),
            "Owed $3.33, Paid $5.00 -> 1 dollar,2 quarters,17 pennies (random)",
        );
    }

    #[test]
    fn verbose_no_change() {
        let tx = Transaction {
            owed_cents: 500,
            paid_cents: 500,
            change_cents: 0,
        };
        assert_eq!(
            format_verbose(&tx, &Vec::new(), &crate::currency::USD, false),
            "Owed $5.00, Paid $5.00 -> no change",
        );
    }

    #[test]
    fn verbose_eur_uses_euro_symbol() {
        let tx = Transaction {
            owed_cents: 150,
            paid_cents: 200,
            change_cents: 50,
        };
        let breakdown = vec![(
            Denomination {
                cents: 50,
                singular: "50 cent coin",
                plural: "50 cent coins",
            },
            1,
        )];
        assert_eq!(
            format_verbose(&tx, &breakdown, &crate::currency::EUR, false),
            "Owed €1.50, Paid €2.00 -> 1 50 cent coin",
        );
    }
}
