use std::{
    io::{Read, stdin},
    ops::RangeInclusive,
};

pub fn parse_input() -> impl Iterator<Item = RangeInclusive<u64>> {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input!");
    let input = input.trim();

    input
        .split(",")
        .map(|part| {
            let (start, end) = part.split_once("-").expect("Couldn't split part!");
            let start = start
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Failed to parse int: {:?}", start));
            let end = end
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse int: {:?}", end));

            start..=end
        })
        .collect::<Vec<_>>()
        .into_iter()
}
