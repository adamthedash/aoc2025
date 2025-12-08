use day8::{KDTree, parse_input};
use std::collections::HashSet;

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
    let mut connections = HashSet::new();
    for _ in 0..1000 {
        let (i, j, _) = neighbours
            .iter_mut()
            .enumerate()
            // Iterate any NNs forward that are currently pointing to an exisitng connection
            // Any that run out of neighbours (i.e. fully connected), are skipped
            .flat_map(|(i, n)| {
                while n
                    .next_if(|&(j, _)| {
                        // Undirected connections
                        let key = if i < j { (i, j) } else { (j, i) };
                        connections.contains(&key)
                    })
                    .is_some()
                {}

                n.peek().map(|&(j, dist2)| (i, j, dist2))
            })
            // Smallest connection length
            .min_by_key(|(_, _, dist2)| *dist2)
            .expect("Ran out of things to connect!");

        // println!(
        //     "Connecting {i} - {j}: {:?} {:?}: {dist2}",
        //     coords[i], coords[j]
        // );

        let key = if i < j { (i, j) } else { (j, i) };
        connections.insert(key);
    }

    // Build circuits
    let mut circuits: Vec<HashSet<usize>> = vec![];
    for (i, j) in connections {
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
    }

    // println!("{:?}", circuits);

    // Get 3 largest circuits
    let mut circuit_lengths = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
    circuit_lengths.sort_unstable();
    circuit_lengths.reverse();
    let answer = circuit_lengths[..3].iter().product::<usize>();

    println!("{answer}");
}
