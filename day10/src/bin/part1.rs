use day10::parse_input;

fn main() {
    let input = parse_input();

    let answer = input
        .map(|(light, buttons, _)| {
            // Try all possible button combos. Pressing a button twice or more is pointless, so only
            // try 0 or once each
            (0_u64..(1 << buttons.len()))
                .map(|button_combo| {
                    // Press the buttons in this combo, starting from all off
                    let result = buttons
                        .iter()
                        .enumerate()
                        // TODO: Could make this better with u64::trailing_zeros()
                        .fold(0, |mut lights, (i, button)| {
                            if (button_combo >> i) & 1 == 1 {
                                lights ^= *button;
                            }

                            lights
                        });

                    (result, button_combo.count_ones())
                })
                // Ignore combos that don't give us the right answer
                .filter_map(|(result, presses)| (result == light).then_some(presses))
                // Find the one with least presses
                .min()
                .expect("No solution found!")
        })
        .sum::<u32>();

    println!("{answer}");
}
