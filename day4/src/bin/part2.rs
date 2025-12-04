use std::collections::VecDeque;

use day4::{neighbourhood, parse_input, print_grid, print_sat, summed_area_table};

fn calc_neighbourhoods(sat: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut neighbourhoods = vec![];
    for (i, row) in sat.iter().enumerate() {
        let mut neighbour_row = vec![];
        for j in 0..row.len() {
            let neighbours = neighbourhood(sat, [i, j], 1);
            neighbour_row.push(neighbours);
        }
        neighbourhoods.push(neighbour_row);
    }

    neighbourhoods
}

fn main() {
    // Neighbourhood size
    const N: usize = 1;

    let mut grid = parse_input();
    let sat = summed_area_table(&grid);
    let mut neighours = calc_neighbourhoods(&sat);

    // Breadth-first search for rolls to remove
    let mut to_check = VecDeque::new();

    // Add initial rolls
    for (i, row) in grid.iter().enumerate() {
        for (j, is_roll) in row.iter().enumerate() {
            if *is_roll {
                to_check.push_back((i, j));
            }
        }
    }

    let mut removed = 0;
    while let Some((i, j)) = to_check.pop_front() {
        // If roll is accessible
        if grid[i][j] && neighours[i][j] <= 4 {
            // Remove this roll
            grid[i][j] = false;
            removed += 1;

            // Update neighbourhood
            for ii in i.saturating_sub(N)..(i + N + 1).min(grid.len()) {
                for jj in j.saturating_sub(N)..(j + N + 1).min(grid[ii].len()) {
                    neighours[ii][jj] -= 1;
                    // Check this spot later in case it has become accessible
                    to_check.push_back((ii, jj));
                }
            }
        }
    }

    println!("{}", removed);
}
