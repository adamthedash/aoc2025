use std::io::stdin;

pub fn parse_input() -> impl Iterator<Item = i32> {
    stdin().lines().map_while(Result::ok).map(|line| {
        let num = line[1..].parse::<i32>().expect("Failed to parse number");
        match line.chars().next().unwrap() {
            'L' => -num,
            'R' => num,
            _ => unreachable!(),
        }
    })
}
