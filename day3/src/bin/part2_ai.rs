use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut total_joltage: u64 = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n = digits.len();
        let k = 12;

        let mut result = Vec::new();
        let mut start = 0;

        for pos in 0..k {
            let end = n - (k - pos);
            let mut max_digit = 0;
            let mut max_index = start;

            for i in start..=end {
                if digits[i] > max_digit {
                    max_digit = digits[i];
                    max_index = i;
                }
            }

            result.push(max_digit);
            start = max_index + 1;
        }

        let joltage: u64 = result.iter().fold(0, |acc, &d| acc * 10 + d as u64);
        total_joltage += joltage;
    }

    println!("{}", total_joltage);
}
