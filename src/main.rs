use std::env;
use std::fs;
use std::process;

use rand::rngs::StdRng;
use rand::SeedableRng;

use cash_register::currency::{USD, EUR};
use cash_register::format::{format_breakdown, format_verbose};
use cash_register::parse::parse_input;
use cash_register::rules::make_change_for;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cash-register <input-file> [--divisor N] [--seed N] [--currency USD|EUR] [--verbose]");
        process::exit(1);
    }

    let file_path = &args[1];
    let divisor: u32 = parse_flag(&args, "--divisor").unwrap_or(3);
    let seed: Option<u64> = parse_flag(&args, "--seed");
    let currency_name = parse_string_flag(&args, "--currency").unwrap_or("USD".to_string());
    let verbose = args.iter().any(|a| a == "--verbose");

    let currency = match currency_name.to_uppercase().as_str() {
        "USD" => &USD,
        "EUR" => &EUR,
        other => {
            eprintln!("Unknown currency: {other}. Supported: USD, EUR");
            process::exit(1);
        }
    };

    let input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading {file_path}: {e}");
            process::exit(1);
        }
    };

    let mut had_error = false;

    // Use a concrete StdRng regardless — seeded or from entropy.
    // This avoids Box<dyn Rng> and keeps everything monomorphized.
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    for result in parse_input(&input) {
        match result {
            Ok(transaction) => {
                let breakdown = make_change_for(&transaction, currency, divisor, &mut rng);
                if verbose {
                    let is_random = divisor > 0 && transaction.owed_cents % divisor == 0;
                    println!("{}", format_verbose(&transaction, &breakdown, is_random));
                } else {
                    println!("{}", format_breakdown(&breakdown));
                }
            }
            Err(e) => {
                eprintln!("{e}");
                had_error = true;
            }
        }
    }

    if had_error {
        process::exit(2);
    }
}

/// Parse a `--flag value` pair from command-line args.
fn parse_flag<T: std::str::FromStr>(args: &[String], flag: &str) -> Option<T> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}

/// Parse a `--flag value` pair as a string.
fn parse_string_flag(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|v| v.clone())
}
