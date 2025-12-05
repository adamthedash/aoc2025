use day5::parse_input;

fn main() {
    let (ranges, ids) = parse_input();

    let answer = ids
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();
    println!("{answer}");
}
