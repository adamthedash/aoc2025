use day3::parse_input;

fn highest_joltage(bank: &[u32]) -> u64 {
    const NUM_BATTERIES: usize = 12;

    // Annotate positions
    let bank = bank
        .iter()
        .map(|j| *j as u64)
        .enumerate()
        .collect::<Vec<_>>();

    let mut joltage = 0;
    let mut start_pos = 0;
    for buffer in (0..NUM_BATTERIES).rev() {
        // Attempt to take the largest digit from the left, leaving enough room for future
        // batteries
        let (pos, digit) = bank[start_pos..bank.len() - buffer]
            .iter()
            // Max returns last element in draw
            .rev()
            .max_by_key(|(_, j)| j)
            .unwrap();

        joltage *= 10;
        joltage += *digit;
        start_pos = pos + 1;
    }

    joltage
}

fn main() {
    let banks = parse_input();

    let answer = banks.map(|bank| highest_joltage(&bank)).sum::<u64>();

    println!("{}", answer);
}
