use std::{io::stdin, ops::RangeInclusive};

pub fn parse_input() -> (Vec<RangeInclusive<u64>>, impl Iterator<Item = u64>) {
    let mut lines = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string());

    let ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Failed to split range");
            let start = start.parse().expect("Failed to parse int!");
            let end = end.parse().expect("Failed to parse int!");

            start..=end
        })
        .collect::<Vec<_>>();

    let ids = lines.map(|line| line.parse().expect("Failed to parse int!"));

    (ranges, ids)
}
