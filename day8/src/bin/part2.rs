use std::collections::HashSet;

use day8::{KDTree, parse_input};

fn main() {
    let coords = parse_input();
    // println!("{:?}", coords);

    let tree = KDTree::build(&coords);
    // println!("{:#?}", tree);

    let mut neighbours = coords
        .iter()
        // Skip 1 because we can't connect to ourselves
        .map(|point| tree.nearest_neighbours(point).skip(1).peekable())
        .collect::<Vec<_>>();

    // Connect shortest boxes
    let mut circuits = (0..coords.len())
        .map(std::iter::once)
        .map(HashSet::from_iter)
        .collect::<Vec<HashSet<_>>>();

    let mut connections = vec![];
    while circuits.len() > 1 {
        // Find the next useful connection to be made
        let (i, j, _) = neighbours
            .iter_mut()
            .enumerate()
            // Iterate any NNs forward that are currently pointing to an exisitng connection
            // Any that run out of neighbours (i.e. fully connected), are skipped
            .flat_map(|(i, n)| {
                while n
                    .next_if(|(j, _)| circuits.iter().any(|c| c.contains(&i) && c.contains(j)))
                    .is_some()
                {}

                n.peek().map(|&(j, dist2)| (i, j, dist2))
            })
            // Smallest connection length
            .min_by_key(|(_, _, dist2)| *dist2)
            .expect("Ran out of things to connect!");

        // Connect the circuits
        // Pop existing circuits connected to either of these boxes
        let i_circuit = circuits
            .iter()
            .position(|circuit| circuit.contains(&i))
            .map(|i| circuits.swap_remove(i).into_iter());
        let j_circuit = circuits
            .iter()
            .position(|circuit| circuit.contains(&j))
            .map(|i| circuits.swap_remove(i).into_iter());

        // Merge em
        let iter = [i_circuit, j_circuit]
            .into_iter()
            .flatten()
            .flatten()
            .chain([i, j]);
        let new_circuit = HashSet::from_iter(iter);

        circuits.push(new_circuit);

        connections.push((i, j));
    }

    // Measure X distance between final connection
    let &(i, j) = connections.last().unwrap();
    let answer = coords[i][0] * coords[j][0];
    println!("{answer}");
}
