use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return;
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let lines: Vec<String> = lines
        .iter()
        .map(|line| format!("{:<width$}", line, width = width))
        .collect();

    let mut separators = vec![];
    for col in 0..width {
        let all_spaces = lines
            .iter()
            .all(|line| line.chars().nth(col).unwrap_or(' ') == ' ');
        if all_spaces {
            separators.push(col);
        }
    }

    let mut ranges = vec![];
    let mut start = 0;
    for &sep in &separators {
        if sep > start {
            ranges.push(start..sep);
        }
        start = sep + 1;
    }
    if start < width {
        ranges.push(start..width);
    }

    let mut grand_total: i64 = 0;

    for range in ranges {
        let problem_lines: Vec<String> = lines
            .iter()
            .map(|line| line[range.clone()].to_string())
            .collect();

        if problem_lines.is_empty() {
            continue;
        }

        let operator_line = &problem_lines[problem_lines.len() - 1];
        let operator_char = operator_line.chars().find(|&c| c == '*' || c == '+');

        let Some(operator) = operator_char else {
            continue;
        };

        let mut numbers = vec![];
        for i in 0..problem_lines.len() - 1 {
            let num_str = problem_lines[i].trim();
            if !num_str.is_empty() {
                if let Ok(num) = num_str.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        if numbers.is_empty() {
            continue;
        }

        let result = if operator == '*' {
            numbers.iter().product::<i64>()
        } else {
            numbers.iter().sum::<i64>()
        };

        grand_total += result;
    }

    println!("{}", grand_total);
}
