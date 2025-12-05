use std::ops::RangeInclusive;

use day5::parse_input;

fn overlaps(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> bool {
    // AABB collision test
    r1.start() <= r2.end() && r1.end() >= r2.start()
}

fn merge(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    *(r1.start().min(r2.start()))..=*(r1.end().max(r2.end()))
}

fn main() {
    let (ranges, _) = parse_input();

    // Merge overlapping ranges
    // https://github.com/adamthedash/saturating_buffer/blob/master/src/saturating_reader.rs
    let mut merged = vec![];
    for mut r1 in ranges {
        // Repeatedly merge with existing ranges
        while let Some(i) = merged.iter().position(|r2| overlaps(&r1, r2)) {
            r1 = merge(&r1, &merged.swap_remove(i));
        }
        merged.push(r1);
    }

    // Count fresh ingredients
    let answer = merged.into_iter().map(|r| r.count()).sum::<usize>();
    println!("{answer}");
}
