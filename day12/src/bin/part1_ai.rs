use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    cells: Vec<(i32, i32)>,
}

impl Shape {
    fn rotate(&self) -> Shape {
        Shape {
            cells: self.cells.iter().map(|&(x, y)| (-y, x)).collect(),
        }
    }

    fn flip(&self) -> Shape {
        Shape {
            cells: self.cells.iter().map(|&(x, y)| (-x, y)).collect(),
        }
    }

    fn normalize(&self) -> Shape {
        if self.cells.is_empty() {
            return Shape { cells: vec![] };
        }

        let min_x = self.cells.iter().map(|&(x, _)| x).min().unwrap();
        let min_y = self.cells.iter().map(|&(_, y)| y).min().unwrap();

        let mut cells: Vec<_> = self
            .cells
            .iter()
            .map(|&(x, y)| (x - min_x, y - min_y))
            .collect();
        cells.sort();

        Shape { cells }
    }

    fn area(&self) -> usize {
        self.cells.len()
    }
}

fn generate_transformations(shape: &Shape) -> Vec<Shape> {
    let mut unique = HashSet::new();
    let mut current = shape.clone();

    for _ in 0..2 {
        for _ in 0..4 {
            let normalized = current.normalize();
            unique.insert(normalized);
            current = current.rotate();
        }
        current = current.flip();
    }

    unique.into_iter().collect()
}

fn can_place(grid: &[Vec<bool>], shape: &Shape, x: i32, y: i32) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for &(dx, dy) in &shape.cells {
        let nx = x + dx;
        let ny = y + dy;

        if nx < 0 || ny < 0 || nx >= width || ny >= height {
            return false;
        }

        if grid[ny as usize][nx as usize] {
            return false;
        }
    }

    true
}

fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, x: i32, y: i32) {
    for &(dx, dy) in &shape.cells {
        grid[(y + dy) as usize][(x + dx) as usize] = true;
    }
}

fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, x: i32, y: i32) {
    for &(dx, dy) in &shape.cells {
        grid[(y + dy) as usize][(x + dx) as usize] = false;
    }
}

fn count_empty_cells(grid: &[Vec<bool>]) -> usize {
    grid.iter().flatten().filter(|&&cell| !cell).count()
}

fn find_first_empty(grid: &[Vec<bool>]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if !cell {
                return Some((x, y));
            }
        }
    }
    None
}

fn solve(
    grid: &mut [Vec<bool>],
    shapes_to_place: &[(usize, Vec<Shape>)],
    index: usize,
    total_area_needed: usize,
) -> bool {
    if index >= shapes_to_place.len() {
        return true;
    }

    let empty_count = count_empty_cells(grid);
    if empty_count < total_area_needed {
        return false;
    }

    let Some((empty_x, empty_y)) = find_first_empty(grid) else {
        return index >= shapes_to_place.len();
    };

    let (_, transformations) = &shapes_to_place[index];
    let area_needed_after = total_area_needed - transformations[0].area();

    let height = grid.len();
    let width = grid[0].len();

    for transformation in transformations {
        for y in empty_y..height {
            let x_start = if y == empty_y { empty_x } else { 0 };
            for x in x_start..width {
                if !can_place(grid, transformation, x as i32, y as i32) {
                    continue;
                }

                place_shape(grid, transformation, x as i32, y as i32);

                if solve(grid, shapes_to_place, index + 1, area_needed_after) {
                    remove_shape(grid, transformation, x as i32, y as i32);
                    return true;
                }

                remove_shape(grid, transformation, x as i32, y as i32);
            }
        }
    }

    false
}

fn can_fit_presents(width: usize, height: usize, base_shapes: &[Shape], counts: &[usize]) -> bool {
    let mut grid = vec![vec![false; width]; height];
    let mut shapes_to_place = Vec::new();
    let mut total_area = 0;

    for (shape_idx, &count) in counts.iter().enumerate() {
        if count == 0 || shape_idx >= base_shapes.len() {
            continue;
        }

        let transformations = generate_transformations(&base_shapes[shape_idx]);
        let shape_area = transformations[0].area();
        total_area += shape_area * count;

        for _ in 0..count {
            shapes_to_place.push((shape_idx, transformations.clone()));
        }
    }

    let grid_area = width * height;
    if total_area > grid_area {
        return false;
    }

    solve(&mut grid, &shapes_to_place, 0, total_area)
}

fn parse_shape(lines: &[String]) -> Shape {
    let mut cells = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((x as i32, y as i32));
            }
        }
    }

    Shape { cells }.normalize()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut base_shapes = Vec::new();
    let mut regions = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.ends_with(':') {
            let mut shape_lines = Vec::new();
            i += 1;

            while i < lines.len()
                && !lines[i].trim().is_empty()
                && !lines[i].contains(':')
                && !lines[i].contains('x')
            {
                shape_lines.push(lines[i].clone());
                i += 1;
            }

            if !shape_lines.is_empty() {
                base_shapes.push(parse_shape(&shape_lines));
            }
        } else if line.contains('x') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                if dims.len() == 2 {
                    let width: usize = dims[0].parse().unwrap();
                    let height: usize = dims[1].parse().unwrap();

                    let counts: Vec<usize> = parts[1]
                        .trim()
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();

                    regions.push((width, height, counts));
                }
            }
            i += 1;
        } else {
            i += 1;
        }
    }

    let mut result = 0;

    for (width, height, counts) in regions {
        if can_fit_presents(width, height, &base_shapes, &counts) {
            result += 1;
        }
    }

    println!("{}", result);
}
