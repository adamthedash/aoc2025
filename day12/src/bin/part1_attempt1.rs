use day12::shape::Shape;

use day12::parse_input;

fn dfs_build_shape(
    available_shapes: &[Shape],
    current_shape: &Shape,
    current_pieces: &[usize],
    desired_pieces: &[usize],
    region_height: usize,
    region_width: usize,
    shapes_tried: &mut usize,
) -> bool {
    // println!("Shape: {:?}", current_pieces);
    // print_shape(current_shape);

    // See what our deficit is
    let needed_pieces = current_pieces
        .iter()
        .zip(desired_pieces)
        .map(|(c, d)| d - c)
        .collect::<Vec<_>>();

    available_shapes
        .iter()
        // Filter for shapes that we have enough left of in the stack
        .filter(|shape| shape.worth <= needed_pieces)
        // Enumerate every way we can combine this shape
        .flat_map(|shape| current_shape.enumerate_tesselations(shape))
        // Check our new valid pieces
        .any(|shape| {
            *shapes_tried += 1;
            if shapes_tried.is_multiple_of(100000) {
                println!("Shapes tried: {}", shapes_tried);
                println!("{}", shape);
            }

            if !(shape.height() <= region_height && shape.width() <= region_width) {
                // Shape is too big for the region
                return false;
            }

            let new_current_pieces = shape
                .worth
                .iter()
                .zip(current_pieces)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>();

            if new_current_pieces == desired_pieces {
                // Found a solution
                println!("Found solution!");
                println!("{}", shape);
                return true;
            }

            // Recurse!
            dfs_build_shape(
                available_shapes,
                &shape,
                &new_current_pieces,
                desired_pieces,
                region_height,
                region_width,
                shapes_tried,
            )
        })
}

fn main() {
    let (shapes, problems) = parse_input();

    // Extend the base set with their orientations
    let shapes = shapes
        .into_iter()
        .flat_map(|shape| shape.enumerate_orientations().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer = problems
        .map(|(h, w, pieces)| {
            println!("{h}x{w}: {:?}", pieces);

            let mut shapes_tried = 0;

            // Select an arbitrary shape to start with
            let success = shapes
                .iter()
                .filter(|shape| shape.worth <= pieces)
                .any(|shape| {
                    // Go look for a solution
                    dfs_build_shape(
                        &shapes,
                        shape,
                        &shape.worth,
                        &pieces,
                        h,
                        w,
                        &mut shapes_tried,
                    )
                });

            println!("Shaped tried: {}, success: {}", shapes_tried, success);

            success
        })
        .filter(|success| *success)
        .count();

    println!("{answer}");
}
