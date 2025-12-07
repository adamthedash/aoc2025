use std::collections::HashMap;
use std::io::{self, Read};

fn count_timelines(
    grid: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&cached) = memo.get(&(start_row, start_col)) {
        return cached;
    }

    let mut row = start_row;

    loop {
        row += 1;

        if row >= grid.len() {
            memo.insert((start_row, start_col), 1);
            return 1;
        }

        if start_col >= grid[row].len() {
            memo.insert((start_row, start_col), 1);
            return 1;
        }

        let cell = grid[row][start_col];

        if cell == '^' {
            let mut count = 0;

            if start_col > 0 {
                count += count_timelines(grid, row, start_col - 1, memo);
            } else {
                count += 1;
            }

            if start_col + 1 < grid[row].len() {
                count += count_timelines(grid, row, start_col + 1, memo);
            } else {
                count += 1;
            }

            memo.insert((start_row, start_col), count);
            return count;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start_row = 0;
    let mut start_col = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_row = r;
                start_col = c;
            }
        }
    }

    let mut memo = HashMap::new();
    let timelines = count_timelines(&grid, start_row, start_col, &mut memo);
    println!("{}", timelines);
}
