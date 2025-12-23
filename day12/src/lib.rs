pub mod shape;

use std::io::stdin;

use crate::shape::Shape;

pub fn parse_input() -> (Vec<Shape>, impl Iterator<Item = (usize, usize, Vec<usize>)>) {
    let mut lines = stdin().lines().map_while(Result::ok).peekable();

    let mut shapes = vec![];
    while lines.peek().is_some_and(|line| !line.contains('x')) {
        // Read shape and parse into mask
        let shape = lines
            .by_ref()
            // Discard index line
            .skip(1)
            // Discard blank line
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        shapes.push(shape);
    }

    let num_shapes = shapes.len();
    let shapes = shapes
        .into_iter()
        .enumerate()
        .map(|(i, mask)| {
            let mut worth = vec![0; num_shapes];
            worth[i] = 1;

            Shape { mask, worth }
        })
        .collect();

    let regions = lines.map(|line| {
        let (area, presents) = line.split_once(": ").unwrap();
        let (w, h) = area.split_once('x').unwrap();
        let w = w.parse().unwrap();
        let h = h.parse().unwrap();

        let presents = presents
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        (h, w, presents)
    });

    (shapes, regions)
}
