use day7::parse_input;

fn main() {
    let (beams, rows) = parse_input();

    let paths_to = beams.into_iter().map(|b| b as usize).collect::<Vec<_>>();

    let paths_to = rows.fold(paths_to, |mut paths_to, row| {
        for (i, is_splitter) in row.iter().enumerate() {
            if *is_splitter && paths_to[i] > 0 {
                // Split the beam, cascading the number of possible paths to each location
                // Splitters are never at the edge, so don't need to worry about bounds checks
                assert!(i > 0 && i < paths_to.len() - 1);
                paths_to[i - 1] += paths_to[i];
                paths_to[i + 1] += paths_to[i];
                paths_to[i] = 0;
            }
        }

        paths_to
    });

    let total_paths = paths_to.iter().sum::<usize>();
    println!("{total_paths}");
}
