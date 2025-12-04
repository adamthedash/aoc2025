use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut accessible_count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let mut adjacent_count = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if grid[nr as usize][nc as usize] == '@' {
                                adjacent_count += 1;
                            }
                        }
                    }
                }

                if adjacent_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    println!("{}", accessible_count);
}
