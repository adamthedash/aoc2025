use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut position = 50;
    let mut zero_count = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().unwrap();

        if direction == "L" {
            position = (position - distance).rem_euclid(100);
        } else {
            position = (position + distance) % 100;
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}
