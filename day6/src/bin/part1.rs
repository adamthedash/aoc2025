use std::io::stdin;

fn main() {
    let lines = stdin().lines().map_while(Result::ok).collect::<Vec<_>>();

    let ops = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let stacks = lines[..lines.len() - 1]
        .iter()
        // Parse numbers
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().expect("Failed to parse int!"))
        })
        // Assign to corresponding stack
        .fold(vec![vec![]; ops.len()], |mut stacks, nums| {
            for (i, num) in nums.enumerate() {
                stacks[i].push(num);
            }

            stacks
        });

    let answer = stacks
        .into_iter()
        .zip(ops)
        .map(|(stack, op)| match op {
            "+" => stack.iter().sum::<u64>(),
            "*" => stack.iter().product(),
            _ => unreachable!(),
        })
        .sum::<u64>();

    println!("{:?}", answer);
}
