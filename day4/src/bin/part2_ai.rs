use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut total_removed = 0;

    loop {
        let mut accessible_positions = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut adjacent_count = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                            continue;
                        }

                        if grid[nr as usize][nc as usize] == '@' {
                            adjacent_count += 1;
                        }
                    }
                }

                if adjacent_count < 4 {
                    accessible_positions.push((r, c));
                }
            }
        }

        if accessible_positions.is_empty() {
            break;
        }

        for (r, c) in accessible_positions {
            grid[r][c] = '.';
            total_removed += 1;
        }
    }

    println!("{}", total_removed);
}
