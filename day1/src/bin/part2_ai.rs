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

        let crossings = if direction == "L" {
            if position == 0 {
                distance / 100
            } else if distance < position {
                0
            } else {
                1 + (distance - position) / 100
            }
        } else {
            (position + distance) / 100 - position / 100
        };

        zero_count += crossings;

        if direction == "L" {
            position = (position - distance).rem_euclid(100);
        } else {
            position = (position + distance) % 100;
        }
    }

    println!("{}", zero_count);
}
