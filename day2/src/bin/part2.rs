use day2::parse_input;

fn is_invalid(id: u64) -> bool {
    let id = format!("{id}");

    // Check lengths that divide evenly
    let repeat_lengths = (1..id.len())
        .filter(|d| id.len() % d == 0)
        .collect::<Vec<_>>();

    for repeat_length in repeat_lengths {
        // Check each chunk against the start chunk
        let pattern = &id[..repeat_length];
        if (repeat_length..id.len())
            .step_by(repeat_length)
            .all(|i| &id[i..i + repeat_length] == pattern)
        {
            return true;
        }
    }

    false
}

fn main() {
    let input = parse_input();

    let answer = input
        .flat_map(|range| range.filter(|id| is_invalid(*id)))
        .sum::<u64>();

    println!("{answer}");
}
