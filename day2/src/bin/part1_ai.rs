use std::io::{self, Read};

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let input = input.trim();

    let mut total = 0u64;

    for range in input.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }

        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for id in start..=end {
            if is_invalid_id(id) {
                total += id;
            }
        }
    }

    println!("{}", total);
}
