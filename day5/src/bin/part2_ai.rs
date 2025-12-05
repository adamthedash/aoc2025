use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (start, end) in ranges {
        if merged.is_empty() {
            merged.push((start, end));
            continue;
        }

        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        if start <= last_end + 1 {
            merged[last_idx] = (last_start, end.max(last_end));
        } else {
            merged.push((start, end));
        }
    }

    let total_count: u64 = merged.iter().map(|(start, end)| end - start + 1).sum();

    println!("{}", total_count);
}
