use day1::parse_input;

fn main() {
    let inputs = parse_input();

    let clicks = inputs.scan(50, |acc, x| {
        // 0 L5 -> -5 /% 100 -> (-1, 95)
        // 0 R5 -> 5 /% 100 -> (0, 5)
        // 95 R5 -> 100 /% 100 -> (1, 0)
        // 5 L5 -> 0 /% 100 -> (0, 0)
        // 5 L105 -> -100 /% 100 -> (-1, 0)
        // 5 L106 -> -101 /% 100 -> (-2, 99)

        // Remove loops first
        let full_turns = x / 100;
        let remainder = x - full_turns * 100;

        let after_turn = *acc + remainder;

        // Don't count an extra one if leaving 0 to the left
        let mut clicks = full_turns.abs();
        if (*acc != 0 && after_turn <= 0) || after_turn > 99 {
            clicks += 1;
        }

        *acc = after_turn.rem_euclid(100);

        Some(clicks)
    });

    let answer = clicks.sum::<i32>();
    println!("{answer}");
}
