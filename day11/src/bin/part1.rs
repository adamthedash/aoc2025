use day11::parse_input;

/// DFS over all possible paths to the end
fn recurse_connections(connections: &[Vec<usize>], start: usize, end: usize) -> usize {
    if start == end {
        return 1;
    }

    connections[start]
        .iter()
        .map(|to| recurse_connections(connections, *to, end))
        .sum()
}

fn main() {
    let (devices, connections) = parse_input();

    let you = devices
        .iter()
        .position(|d| d == "you")
        .expect("No 'you' device");
    let out = devices
        .iter()
        .position(|d| d == "out")
        .expect("No 'out' device");

    // Turn connections into dense LUT
    let mut connnections_lut = vec![vec![]; devices.len()];
    for (from, to) in &connections {
        // NOTE: Assuming no conflicting/duplicate connections
        connnections_lut[*from].extend_from_slice(to);
    }

    let paths = recurse_connections(&connnections_lut, you, out);
    println!("paths: {}", paths);
}
