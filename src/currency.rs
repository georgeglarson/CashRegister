/// A single denomination: its value in cents and display names.
#[derive(Debug, Clone, Copy)]
pub struct Denomination {
    pub cents: u32,
    pub singular: &'static str,
    pub plural: &'static str,
}

/// A currency configuration: a name, symbol, and denominations (largest first).
#[derive(Debug, Clone)]
pub struct Currency {
    pub name: &'static str,
    pub symbol: &'static str,
    pub denominations: &'static [Denomination],
}

pub static USD: Currency = Currency {
    name: "USD",
    symbol: "$",
    denominations: &[
        Denomination {
            cents: 100,
            singular: "dollar",
            plural: "dollars",
        },
        Denomination {
            cents: 25,
            singular: "quarter",
            plural: "quarters",
        },
        Denomination {
            cents: 10,
            singular: "dime",
            plural: "dimes",
        },
        Denomination {
            cents: 5,
            singular: "nickel",
            plural: "nickels",
        },
        Denomination {
            cents: 1,
            singular: "penny",
            plural: "pennies",
        },
    ],
};

pub static EUR: Currency = Currency {
    name: "EUR",
    symbol: "€",
    denominations: &[
        Denomination {
            cents: 200,
            singular: "2 euro coin",
            plural: "2 euro coins",
        },
        Denomination {
            cents: 100,
            singular: "1 euro coin",
            plural: "1 euro coins",
        },
        Denomination {
            cents: 50,
            singular: "50 cent coin",
            plural: "50 cent coins",
        },
        Denomination {
            cents: 20,
            singular: "20 cent coin",
            plural: "20 cent coins",
        },
        Denomination {
            cents: 10,
            singular: "10 cent coin",
            plural: "10 cent coins",
        },
        Denomination {
            cents: 5,
            singular: "5 cent coin",
            plural: "5 cent coins",
        },
        Denomination {
            cents: 2,
            singular: "2 cent coin",
            plural: "2 cent coins",
        },
        Denomination {
            cents: 1,
            singular: "1 cent coin",
            plural: "1 cent coins",
        },
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usd_denominations_are_sorted_descending() {
        let denoms = USD.denominations;
        for window in denoms.windows(2) {
            assert!(
                window[0].cents > window[1].cents,
                "{} ({}) should be larger than {} ({})",
                window[0].singular,
                window[0].cents,
                window[1].singular,
                window[1].cents,
            );
        }
    }

    #[test]
    fn usd_smallest_denomination_is_one_cent() {
        let last = USD.denominations.last().unwrap();
        assert_eq!(
            last.cents, 1,
            "smallest denomination must be 1 cent for exact change"
        );
    }

    #[test]
    fn eur_smallest_denomination_is_one_cent() {
        let last = EUR.denominations.last().unwrap();
        assert_eq!(last.cents, 1);
    }
}
