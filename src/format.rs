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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Denomination;

    fn penny() -> Denomination {
        Denomination { cents: 1, singular: "penny", plural: "pennies" }
    }

    fn quarter() -> Denomination {
        Denomination { cents: 25, singular: "quarter", plural: "quarters" }
    }

    fn dime() -> Denomination {
        Denomination { cents: 10, singular: "dime", plural: "dimes" }
    }

    fn dollar() -> Denomination {
        Denomination { cents: 100, singular: "dollar", plural: "dollars" }
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
        assert!(!output.contains(", "), "output should not have spaces after commas");
        assert_eq!(output, "3 quarters,1 dime,3 pennies");
    }
}
