use std::process::Command;

fn cargo_bin() -> Command {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--quiet", "--"]);
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd
}

#[test]
fn sample_input_greedy_lines() {
    // Lines where owed is NOT divisible by 3 should produce deterministic output.
    // 2.12 -> 212, 212 % 3 != 0 -> greedy: 88 cents = 3 quarters, 1 dime, 3 pennies
    // 1.97 -> 197, 197 % 3 != 0 -> greedy: 3 cents = 3 pennies
    // 3.33 -> 333, 333 % 3 == 0 -> random (skip checking this line)
    let output = cargo_bin()
        .arg("sample_input.txt")
        .arg("--seed")
        .arg("42")
        .output()
        .expect("failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines.len(), 3, "expected 3 output lines, got: {stdout:?}");
    assert_eq!(lines[0], "3 quarters,1 dime,3 pennies");
    assert_eq!(lines[1], "3 pennies");
    // Line 3 is random — just verify it's non-empty
    assert!(!lines[2].is_empty(), "random line should not be empty");
}

#[test]
fn seed_produces_deterministic_output() {
    let run = |seed: &str| -> String {
        let output = cargo_bin()
            .args(["sample_input.txt", "--seed", seed])
            .output()
            .expect("failed to run binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let first = run("99");
    let second = run("99");
    assert_eq!(first, second, "same seed should produce identical output");
}

#[test]
fn custom_divisor() {
    // With --divisor 0, no lines are random, so all output is greedy
    let output = cargo_bin()
        .args(["sample_input.txt", "--divisor", "0"])
        .output()
        .expect("failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "3 quarters,1 dime,3 pennies");
    assert_eq!(lines[1], "3 pennies");
    // With divisor 0, line 3 should also be greedy: 167 = 1 dollar,2 quarters,1 dime,1 nickel,2 pennies
    assert_eq!(lines[2], "1 dollar,2 quarters,1 dime,1 nickel,2 pennies");
}

#[test]
fn missing_file_returns_nonzero_exit() {
    let output = cargo_bin()
        .arg("nonexistent.txt")
        .output()
        .expect("failed to run binary");

    assert!(!output.status.success());
}

#[test]
fn no_args_shows_usage() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("failed to run binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage"), "should show usage message");
    assert!(!output.status.success());
}

#[test]
fn random_line_sums_correctly() {
    // 3.33,5.00 -> 167 cents of change, owed 333 which is divisible by 3
    // Verify the random output sums to the correct total
    let output = cargo_bin()
        .args(["sample_input.txt", "--seed", "42"])
        .output()
        .expect("failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let random_line = stdout.lines().nth(2).expect("expected 3 lines");

    let total = parse_output_cents(random_line);
    assert_eq!(total, 167, "random change should sum to 167 cents (got {total})");
}

/// Parse an output line like "1 dollar,2 quarters,1 nickel,2 pennies" into total cents.
fn parse_output_cents(line: &str) -> u32 {
    line.split(',')
        .map(|part| {
            let part = part.trim();
            let (count_str, name) = part.split_once(' ').expect("expected 'N name' format");
            let count: u32 = count_str.parse().expect("expected numeric count");
            let cents_per = match name {
                "dollar" | "dollars" => 100,
                "half dollar" | "half dollars" => 50,
                "quarter" | "quarters" => 25,
                "dime" | "dimes" => 10,
                "nickel" | "nickels" => 5,
                "penny" | "pennies" => 1,
                other => panic!("unknown denomination: {other}"),
            };
            count * cents_per
        })
        .sum()
}
