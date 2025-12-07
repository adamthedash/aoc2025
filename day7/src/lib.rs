use std::io::stdin;

pub fn parse_input() -> (Vec<bool>, impl Iterator<Item = Vec<bool>>) {
    let mut input = stdin().lines().map_while(Result::ok);

    // Find where the beam will start
    let start_line = input.by_ref().next().expect("No first line!");
    let start_position = start_line.find('S').expect("No starting position!");
    let beams = (0..start_line.len()).map(|i| i == start_position).collect();

    let splitters = input.map(|line| line.chars().map(|c| c == '^').collect());

    (beams, splitters)
}
