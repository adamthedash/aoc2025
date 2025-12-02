use day2::parse_input;

fn is_invalid(id: u64) -> bool {
    let id = format!("{id}");
    if id.len() % 2 == 1 {
        // Odd length cannot be invalid
        return false;
    }

    let repeat_length = id.len() / 2;
    id[..repeat_length] == id[repeat_length..]
}

fn main() {
    let input = parse_input();

    let answer = input
        .flat_map(|range| range.filter(|id| is_invalid(*id)))
        .sum::<u64>();

    println!("{answer}");
}
