pub mod constraints;
pub mod gaussian_elimination;

use std::io::stdin;

pub fn parse_input() -> impl Iterator<Item = (u64, Vec<u64>, Vec<u64>)> {
    stdin().lines().map_while(Result::ok).map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let lights = parts[0];
        let buttons = &parts[1..parts.len() - 1];
        let joltages = parts[parts.len() - 1];

        // Turn light indicators into bitmask
        let lights = lights[1..lights.len() - 1]
            .char_indices()
            .fold(0, |mut mask, (i, c)| {
                if c == '#' {
                    mask |= 1 << i;
                }

                mask
            });

        // Turn buttons into bit masks
        let buttons = buttons
            .iter()
            .map(|button| {
                let mut pattern = 0;
                button[1..button.len() - 1]
                    .split(',')
                    .map(|light| light.parse::<u64>().expect("Failed to parse int!"))
                    .for_each(|light| {
                        pattern |= 1 << light;
                    });

                pattern
            })
            .collect::<Vec<_>>();

        // Parse joltages
        let joltages = joltages[1..joltages.len() - 1]
            .split(',')
            .map(|j| j.parse().expect("Failed to parse int!"))
            .collect::<Vec<_>>();

        (lights, buttons, joltages)
    })
}
