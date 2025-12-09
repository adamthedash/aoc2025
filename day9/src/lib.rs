use std::io::stdin;

pub fn parse_input() -> Vec<[u64; 2]> {
    stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let (x, y) = line.split_once(",").expect("Failed to split line!");
            [x, y].map(|p| p.parse().expect("Failed to parse int!"))
        })
        .collect()
}
