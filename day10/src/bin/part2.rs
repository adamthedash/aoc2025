use day10::gaussian_elimination::GaussianElimination;
use day10::{
    constraints::{Constraint, ConstraintSolver},
    parse_input,
};

fn main() {
    let input = parse_input();

    let answer = input
        .map(|(_, mut buttons, joltages)| {
            buttons.sort_unstable();
            buttons.reverse();

            // Pre-compute some stuff for buttons
            let buttons = buttons
                .into_iter()
                .enumerate()
                .map(|(i, button)| {
                    // Maximum constraint
                    let max = joltages
                        .iter()
                        .enumerate()
                        .filter_map(|(i, j)| ((button >> i) & 1 == 1).then_some(*j))
                        .min()
                        .expect("Button doesnt give any joltage!");

                    (i, button, max)
                })
                .collect::<Vec<_>>();

            println!("{:?}", joltages);
            for (i, b, max) in &buttons {
                println!("{i}: {:>10b} <= {}", b, max);
            }
            println!();

            // Construct matrix
            let rows = joltages
                .iter()
                .enumerate()
                .map(|(i, j)| {
                    buttons
                        .iter()
                        .map(|(_, button, _)| (*button >> i) & 1)
                        .chain(std::iter::once(*j))
                        .map(|x| x as i64)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            // for r in &rows {
            //     println!("{:?}", r);
            // }

            // Simplify matrix as much as possible
            let (order, rows) = GaussianElimination::simplify(rows);
            // println!("Simplified:");
            // println!("{:>2?}", order);
            // for r in &rows {
            //     println!("{:>2?}", r);
            // }

            // Domains
            let domains = order
                .iter()
                .map(|i| 0..=buttons[*i].2 as u32)
                .collect::<Vec<_>>();

            // Constraints
            let mut constraints = Constraint::from_matrix(&rows);

            // Re-order variables so that constraints are checked earlier
            let order = (0..domains.len()).rev().collect::<Vec<_>>();

            let domains = order
                .iter()
                .map(|i| domains[*i].clone())
                .collect::<Vec<_>>();

            constraints.iter_mut().for_each(|c| match c {
                Constraint::Polynomial { variables, .. } => {
                    variables
                        .iter_mut()
                        .for_each(|i| *i = order.iter().position(|o| o == i).unwrap());
                }
            });

            let solver = ConstraintSolver::new(domains, constraints);
            let best = solver.iter_solutions();

            if best == u32::MAX {
                panic!("No solution");
            }

            best as i64
        })
        .inspect(|_| println!("------------------------------"))
        .sum::<i64>();

    println!("{answer}");
}
