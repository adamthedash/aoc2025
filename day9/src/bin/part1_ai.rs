use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let mut tiles = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let x: i64 = parts[0].parse().unwrap();
            let y: i64 = parts[1].parse().unwrap();
            tiles.push((x, y));
        }
    }

    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    println!("{}", max_area);
}
