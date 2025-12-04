use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut total_joltage = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut max_joltage = 0;

        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                let joltage = digits[i] * 10 + digits[j];
                max_joltage = max_joltage.max(joltage);
            }
        }

        total_joltage += max_joltage;
    }

    println!("{}", total_joltage);
}
