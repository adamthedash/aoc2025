use day7::parse_input;

fn print_beams(beams: &[bool]) {
    let displayed = beams
        .iter()
        .map(|b| if *b { '|' } else { '.' })
        .collect::<String>();
    println!("{}", displayed);
}

fn main() {
    let (beams, rows) = parse_input();

    let (_, splits) = rows.fold((beams, 0), |(mut beams, mut splits), row| {
        for (i, is_splitter) in row.iter().enumerate() {
            if *is_splitter && beams[i] {
                // Split the beam
                // Splitters are never at the edge, so don't need to worry about bounds checks
                assert!(i > 0 && i < beams.len() - 1);
                beams[i] = false;
                beams[i - 1] = true;
                beams[i + 1] = true;

                splits += 1;
            }
        }
        print_beams(&beams);

        (beams, splits)
    });

    println!("{splits}");
}
