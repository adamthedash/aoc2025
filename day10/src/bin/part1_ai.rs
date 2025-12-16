use std::io::{self, Read};

fn parse_machine(line: &str) -> Option<(Vec<bool>, Vec<Vec<usize>>)> {
    let bracket_start = line.find('[')?;
    let bracket_end = line.find(']')?;

    let target_str = &line[bracket_start + 1..bracket_end];
    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

    let brace_start = line.find('{')?;
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

    Some((target, buttons))
}

fn min_presses_for_machine(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let n_buttons = buttons.len();
    let n_lights = target.len();

    let mut min_presses = usize::MAX;

    for mask in 0..(1 << n_buttons) {
        let mut state = vec![false; n_lights];
        let mut presses = 0;

        for (i, button) in buttons.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                presses += 1;
                for &light_idx in button {
                    if light_idx < n_lights {
                        state[light_idx] = !state[light_idx];
                    }
                }
            }
        }

        if state == target {
            min_presses = min_presses.min(presses);
        }
    }

    min_presses
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut total_presses = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((target, buttons)) = parse_machine(line) {
            let min = min_presses_for_machine(&target, &buttons);
            total_presses += min;
        }
    }

    println!("{}", total_presses);
}
