use std::collections::BinaryHeap;
use std::io::{self, Read};

fn parse_machine(line: &str) -> Option<(Vec<Vec<usize>>, Vec<i64>)> {
    let brace_start = line.find('{')?;
    let brace_end = line.find('}')?;

    let joltage_str = &line[brace_start + 1..brace_end];
    let targets: Vec<i64> = joltage_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let bracket_end = line.find(']')?;
    let buttons_str = &line[bracket_end + 1..brace_start].trim();

    let mut buttons = Vec::new();
    let mut in_paren = false;
    let mut current = String::new();

    for ch in buttons_str.chars() {
        if ch == '(' {
            in_paren = true;
            current.clear();
        } else if ch == ')' {
            in_paren = false;
            if !current.is_empty() {
                let indices: Vec<usize> = current
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                buttons.push(indices);
            }
        } else if in_paren {
            current.push(ch);
        }
    }

    Some((buttons, targets))
}

fn min_presses_for_machine(buttons: &[Vec<usize>], targets: &[i64]) -> i64 {
    let n_counters = targets.len();

    #[derive(Clone, Eq, PartialEq)]
    struct State {
        counters: Vec<i64>,
        presses: Vec<i64>,
        cost: i64,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap = BinaryHeap::new();
    let initial_state = State {
        counters: vec![0; n_counters],
        presses: vec![0; buttons.len()],
        cost: 0,
    };
    heap.push(initial_state);

    let mut best_cost = i64::MAX;
    let max_iterations = 10000000;
    let mut iterations = 0;

    while let Some(state) = heap.pop() {
        iterations += 1;
        if iterations > max_iterations {
            break;
        }

        if state.counters == *targets {
            return state.cost;
        }

        if state.cost >= best_cost {
            continue;
        }

        for (button_idx, button) in buttons.iter().enumerate() {
            let mut helps = false;
            let mut overshoots = false;

            for &counter_idx in button {
                if counter_idx < n_counters {
                    if state.counters[counter_idx] < targets[counter_idx] {
                        helps = true;
                    }
                    if state.counters[counter_idx] >= targets[counter_idx] {
                        overshoots = true;
                    }
                }
            }

            if !helps || overshoots {
                continue;
            }

            let mut new_counters = state.counters.clone();
            for &counter_idx in button {
                if counter_idx < n_counters {
                    new_counters[counter_idx] += 1;
                }
            }

            let mut new_presses = state.presses.clone();
            new_presses[button_idx] += 1;

            let new_cost = state.cost + 1;

            if new_cost < best_cost {
                heap.push(State {
                    counters: new_counters,
                    presses: new_presses,
                    cost: new_cost,
                });
            }
        }
    }

    best_cost
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut total_presses = 0i64;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((buttons, targets)) = parse_machine(line) {
            let min = min_presses_for_machine(&buttons, &targets);
            if min == i64::MAX {
                eprintln!("No solution found");
            } else {
                total_presses += min;
            }
        }
    }

    println!("{}", total_presses);
}
