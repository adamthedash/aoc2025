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

    let mut memo: HashMap<(String, bool, bool), usize> = HashMap::new();
    let count = dfs(&graph, "svr", "out", false, false, &mut memo);
    println!("{}", count);
}

fn dfs(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    mut seen_dac: bool,
    mut seen_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if current == "dac" {
        seen_dac = true;
    }
    if current == "fft" {
        seen_fft = true;
    }

    if current == target {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    let memo_key = (current.to_string(), seen_dac, seen_fft);
    if let Some(&cached) = memo.get(&memo_key) {
        return cached;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += dfs(graph, neighbor, target, seen_dac, seen_fft, memo);
        }
    }

    memo.insert(memo_key, total_paths);
    total_paths
}
