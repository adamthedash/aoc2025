use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let device = parts[0].to_string();
            let outputs: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
            graph.insert(device, outputs);
        }
    }

    let count = count_paths(&graph, "you", "out");
    println!("{}", count);
}

fn count_paths(graph: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
    let mut visited = HashMap::new();
    dfs(graph, start, end, &mut visited)
}

fn dfs(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashMap<String, bool>,
) -> usize {
    if current == target {
        return 1;
    }

    visited.insert(current.to_string(), true);

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if !visited.contains_key(neighbor.as_str()) {
                total_paths += dfs(graph, neighbor, target, visited);
            }
        }
    }

    visited.remove(current);

    total_paths
}
