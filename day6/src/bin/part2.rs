use std::io::stdin;

fn main() {
    let lines = stdin().lines().map_while(Result::ok).collect::<Vec<_>>();

    // Figure out what the padding size is using last row
    // We know the operator is always aligned with the beginning of each problem
    let mut problem_widths = lines
        .last()
        .unwrap()
        .split(|c| c != ' ')
        .skip(1)
        .map(str::len)
        .collect::<Vec<_>>();
    *problem_widths.last_mut().unwrap() += 1;

    // Create column spans
    let spans = problem_widths.into_iter().scan(0, |acc, width| {
        let start = *acc;
        let end = start + width;
        *acc += width + 1;

        Some(start..end)
    });

    let answer = spans
        .map(|span| {
            // Read operator
            let op = &lines.last().unwrap()[span.start..span.start + 1];

            // Read number columns and parse them
            let stack = span.map(|i| {
                let num = lines[..lines.len() - 1]
                    .iter()
                    .map(|line| &line[i..i + 1])
                    .collect::<Vec<_>>()
                    .concat();

                num.trim().parse::<u64>().expect("Failed to parse int!")
            });

            // Solve this problem
            match op {
                "+" => stack.sum::<u64>(),
                "*" => stack.product(),
                _ => unreachable!(),
            }
        })
        .sum::<u64>();

    println!("{answer}");
}
