use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let mut red_tiles = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let x: i64 = parts[0].parse().unwrap();
            let y: i64 = parts[1].parse().unwrap();
            red_tiles.push((x, y));
        }
    }

    let mut rectangles = Vec::new();

    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let area = width * height;

            rectangles.push((area, min_x, max_x, min_y, max_y));
        }
    }

    rectangles.sort_by(|a, b| b.0.cmp(&a.0));

    for (area, min_x, max_x, min_y, max_y) in rectangles {
        if is_rectangle_valid(min_x, max_x, min_y, max_y, &red_tiles) {
            println!("{}", area);
            return;
        }
    }

    println!("0");
}

fn is_rectangle_valid(
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    polygon: &[(i64, i64)],
) -> bool {
    let width = max_x - min_x;
    let height = max_y - min_y;

    let samples_per_edge = ((width.max(height) / 50).max(20).min(500)) as usize;

    for i in 0..=samples_per_edge {
        let t = i as f64 / samples_per_edge as f64;

        let x_interp = min_x as f64 + t * width as f64;
        let y_interp = min_y as f64 + t * height as f64;

        let top = (x_interp.round() as i64, min_y);
        let bottom = (x_interp.round() as i64, max_y);
        let left = (min_x, y_interp.round() as i64);
        let right = (max_x, y_interp.round() as i64);

        if !is_point_inside_or_on_boundary(top, polygon) {
            return false;
        }
        if !is_point_inside_or_on_boundary(bottom, polygon) {
            return false;
        }
        if !is_point_inside_or_on_boundary(left, polygon) {
            return false;
        }
        if !is_point_inside_or_on_boundary(right, polygon) {
            return false;
        }
    }

    let interior_samples = ((width.max(height) / 100).max(5).min(50)) as usize;
    for i in 1..interior_samples {
        for j in 1..interior_samples {
            let x = min_x + (i as i64 * width) / interior_samples as i64;
            let y = min_y + (j as i64 * height) / interior_samples as i64;
            if !is_point_inside_or_on_boundary((x, y), polygon) {
                return false;
            }
        }
    }

    true
}

fn is_point_inside_or_on_boundary(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    for i in 0..polygon.len() {
        let start = polygon[i];
        let end = polygon[(i + 1) % polygon.len()];
        if is_on_segment(point, start, end) {
            return true;
        }
    }

    is_inside_polygon(point, polygon)
}

fn is_on_segment(point: (i64, i64), start: (i64, i64), end: (i64, i64)) -> bool {
    let (x, y) = point;
    let (x1, y1) = start;
    let (x2, y2) = end;

    if x1 == x2 {
        x == x1 && y >= y1.min(y2) && y <= y1.max(y2)
    } else if y1 == y2 {
        y == y1 && x >= x1.min(x2) && x <= x1.max(x2)
    } else {
        false
    }
}

fn is_inside_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;
    let mut inside = false;

    let n = polygon.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
    }

    inside
}
