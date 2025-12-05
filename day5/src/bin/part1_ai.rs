use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let available_ids: Vec<u64> = parts[1].lines().map(|line| line.parse().unwrap()).collect();

    let fresh_count = available_ids
        .iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count();

    println!("{}", fresh_count);
}
