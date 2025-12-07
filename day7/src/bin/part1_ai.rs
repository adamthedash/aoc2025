use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

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

    let mut splits = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut splitters_hit = HashSet::new();

    queue.push_back((start_row, start_col));
    visited.insert((start_row, start_col));

    while let Some((start_r, start_c)) = queue.pop_front() {
        let mut row = start_r;

        loop {
            row += 1;

            if row >= grid.len() {
                break;
            }

            if start_c >= grid[row].len() {
                break;
            }

            let cell = grid[row][start_c];

            if cell == '^' {
                println!("{} {}", row, start_c);
                if !splitters_hit.contains(&(row, start_c)) {
                    splitters_hit.insert((row, start_c));
                    splits += 1;
                    println!("+");
                }

                if start_c > 0 && !visited.contains(&(row, start_c - 1)) {
                    visited.insert((row, start_c - 1));
                    queue.push_back((row, start_c - 1));
                }

                if start_c + 1 < grid[row].len() && !visited.contains(&(row, start_c + 1)) {
                    visited.insert((row, start_c + 1));
                    queue.push_back((row, start_c + 1));
                }

                break;
            }
        }
    }

    println!("{}", splits);
}
