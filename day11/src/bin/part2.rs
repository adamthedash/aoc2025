use std::collections::HashMap;

use day11::parse_input;

/// Perform a sparse matrix square
fn square_connections(
    connections: &[Vec<usize>],
    paths: &HashMap<(usize, usize), usize>,
) -> Option<(Vec<Vec<usize>>, HashMap<(usize, usize), usize>)> {
    let (out_paths, out_connections) = mul_connections(connections, connections, paths, paths);

    if out_connections.values().sum::<usize>() == 0 {
        None
    } else {
        Some((out_paths, out_connections))
    }
}

/// Perform a sparse matrix multiplication
fn mul_connections(
    connections1: &[Vec<usize>],
    connections2: &[Vec<usize>],
    paths1: &HashMap<(usize, usize), usize>,
    paths2: &HashMap<(usize, usize), usize>,
) -> (Vec<Vec<usize>>, HashMap<(usize, usize), usize>) {
    let mut out_connections = vec![vec![]; connections1.len()];
    let mut out_paths = HashMap::new();

    for start in 0..connections1.len() {
        for hop in &connections1[start] {
            let hop1_paths = paths1[&(start, *hop)];
            for end in &connections2[*hop] {
                out_connections[start].push(*end);

                let new_paths = hop1_paths * paths2[&(*hop, *end)];
                *out_paths.entry((start, *end)).or_default() += new_paths;
            }
        }
    }

    // Sort and dedup
    for hops in out_connections.iter_mut() {
        hops.sort_unstable();
        hops.dedup();
    }

    (out_connections, out_paths)
}

fn main() {
    let (devices, connections) = parse_input();

    // Points of interest
    let svr = devices
        .iter()
        .position(|d| d == "svr")
        .expect("No 'svr' device");
    let dac = devices
        .iter()
        .position(|d| d == "dac")
        .expect("No 'dac' device");
    let fft = devices
        .iter()
        .position(|d| d == "fft")
        .expect("No 'fft' device");
    let out = devices
        .iter()
        .position(|d| d == "out")
        .expect("No 'out' device");

    // Turn connections into dense LUT
    // There are no cycles in the connections
    let mut connections_lut = vec![vec![]; devices.len()];
    let mut connections_num_paths = HashMap::new();
    for (from, to) in &connections {
        connections_lut[*from].extend_from_slice(to);
        for to in to {
            connections_num_paths.insert((*from, *to), 1);
        }
    }

    // Compute a power series of connection matrices
    // This is used to quickly compute any matrix power we want
    // Not actually sure if this is faster than just having a running A^i
    let power_two_connections =
        std::iter::successors(Some((connections_lut, connections_num_paths)), |(m, p)| {
            square_connections(m, p)
        })
        .collect::<Vec<_>>();

    // Enumerate all possible path lengths
    let mut all_paths = HashMap::<_, usize>::new();
    for i in 1..(1 << power_two_connections.len()) {
        // Build starting points - fake entry point connections
        // We only care about starting from specific points, the rest can be skipped to save
        // compute
        let mut start = vec![vec![]; devices.len()];
        let mut paths = HashMap::new();
        for i in [svr, fft, dac] {
            start[i].push(i);
            paths.insert((i, i), 1);
        }

        // Build full length connections
        let (_, full_paths) = power_two_connections
            .iter()
            .enumerate()
            // Use the bitmask to select which matrices to use
            .flat_map(|(e, connections)| ((i >> e) & 1 == 1).then_some(connections))
            // Sum the series of matrices
            .fold((start, paths), |(connections, paths), (c, p)| {
                mul_connections(&connections, c, &paths, p)
            });

        for (k, v) in full_paths {
            *all_paths.entry(k).or_default() += v;
        }
    }

    // svr -> dac -> fft -> out
    // svr -> fft -> dac -> out
    let combos = [
        [(svr, dac), (dac, fft), (fft, out)],
        [(svr, fft), (fft, dac), (dac, out)],
    ];

    let answer = combos
        .iter()
        .map(|hops| {
            hops.iter()
                .map(|(from, to)| all_paths.get(&(*from, *to)).copied().unwrap_or_default())
                .product::<usize>()
        })
        .sum::<usize>();

    println!("{answer}");
}
