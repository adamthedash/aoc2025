use day9::parse_input;

fn main() {
    let tiles = parse_input();

    let pairs = tiles
        .iter()
        .enumerate()
        .take(tiles.len() - 1)
        .flat_map(|(i, t1)| tiles.iter().skip(i + 1).map(move |t2| (t1, t2)));

    let answer = pairs
        .map(|(t1, t2)| (t1[0].abs_diff(t2[0]) + 1) * (t1[1].abs_diff(t2[1]) + 1))
        .max()
        .unwrap();

    println!("{answer}");
}
