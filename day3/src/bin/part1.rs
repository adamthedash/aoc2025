use day3::parse_input;

fn highest_joltage(bank: &[u32]) -> u32 {
    let (i, left_digit) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        // Max returns last element in draw
        .rev()
        .max_by_key(|(_, j)| **j)
        .unwrap();

    let right_digit = bank[i + 1..].iter().max().unwrap();

    *left_digit * 10 + *right_digit
}

fn main() {
    let banks = parse_input();

    let answer = banks.map(|bank| highest_joltage(&bank)).sum::<u32>();

    println!("{}", answer);
}
