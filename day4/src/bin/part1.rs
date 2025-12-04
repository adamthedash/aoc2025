use day4::{neighbourhood, parse_input, summed_area_table};

fn main() {
    let grid = parse_input();
    let sat = summed_area_table(&grid);

    let answer = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                // Only look at grid cells with rolls
                .filter(|(_, is_roll)| **is_roll)
                .map(move |(j, _)| [i, j])
        })
        // Up to 4 adjacent rolls including self
        .filter(|point| neighbourhood(&sat, *point, 1) <= 4)
        .count();

    println!("{}", answer);
}
