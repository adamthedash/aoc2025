use std::io::stdin;

pub fn parse_input() -> Vec<Vec<bool>> {
    stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().chars().map(|c| c == '@').collect())
        .collect()
}

// https://en.wikipedia.org/wiki/Summed-area_table
pub fn summed_area_table(grid: &[Vec<bool>]) -> Vec<Vec<u32>> {
    let mut sat: Vec<Vec<u32>> = vec![];
    for (i, row) in grid.iter().enumerate() {
        let mut sat_row = vec![];
        for (j, val) in row.iter().enumerate() {
            let (left, up, left_up) = match (i == 0, j == 0) {
                (true, true) => (0, 0, 0),
                (true, false) => (sat_row[j - 1], 0, 0),
                (false, true) => (0, sat[i - 1][j], 0),
                (false, false) => (sat_row[j - 1], sat[i - 1][j], sat[i - 1][j - 1]),
            };

            sat_row.push(*val as u32 + left + up - left_up);
        }
        sat.push(sat_row);
    }

    sat
}

// Sum of values around a point
pub fn neighbourhood(sat: &[Vec<u32>], point: [usize; 2], n: usize) -> u32 {
    let shape = [sat.len(), sat[0].len()];

    let max = point
        .map(|p| p + n)
        .into_iter()
        .zip(shape)
        .map(|(a, b)| a.min(b - 1))
        .collect::<Vec<_>>();

    let min = point.map(|p| p.saturating_sub(n + 1));

    let (left, up, left_up) = match point.map(|p| p < n + 1) {
        [true, true] => (0, 0, 0),
        [true, false] => (sat[max[0]][min[1]], 0, 0),
        [false, true] => (0, sat[min[0]][max[1]], 0),
        [false, false] => (
            sat[max[0]][min[1]],
            sat[min[0]][max[1]],
            sat[min[0]][min[1]],
        ),
    };

    sat[max[0]][max[1]] + left_up - left - up
}

pub fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for x in row {
            if *x {
                print!("{:>3}", "@");
            } else {
                print!("{:>3}", ".");
            }
        }
        println!();
    }
}

pub fn print_sat(sat: &[Vec<u32>]) {
    for row in sat {
        for x in row {
            print!("{x:>3}");
        }
        println!();
    }
}

pub fn print_neighbours(sat: &[Vec<u32>], range: usize) {
    for (i, row) in sat.iter().enumerate() {
        for j in 0..row.len() {
            let neighbours = neighbourhood(sat, [i, j], range);
            print!("{:>3}", neighbours);
        }
        println!();
    }
}
