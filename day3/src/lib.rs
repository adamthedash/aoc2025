use std::io::stdin;

pub fn parse_input() -> impl Iterator<Item = Vec<u32>> {
    stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().flat_map(|c| c.to_digit(10)).collect())
}
