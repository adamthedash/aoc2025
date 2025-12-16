use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub enum Constraint {
    /// ax + by = c
    Polynomial {
        variables: Vec<usize>,
        weights: Vec<i64>,
        value: i64,
    },
}

impl Constraint {
    /// Checks if the constraint holds with the provided candidate variable values
    fn evaluate(&self, candidates: &[u32]) -> bool {
        debug_assert!(self.is_checkable(candidates));

        use Constraint::*;
        match self {
            Polynomial {
                variables,
                weights,
                value,
            } => {
                variables
                    .iter()
                    .zip(weights)
                    .map(|(i, w)| candidates[*i] as i64 * w)
                    .sum::<i64>()
                    == *value
            }
        }
    }

    /// Checks if all the required variables are assigned
    fn is_checkable(&self, candidates: &[u32]) -> bool {
        use Constraint::*;
        match self {
            Polynomial { variables, .. } => variables.iter().all(|i| candidates.len() > *i),
        }
    }

    /// Create set of constraints from a row-echelon form matrix
    pub fn from_matrix(rows: &[Vec<i64>]) -> Vec<Self> {
        rows.iter()
            .map(|row| {
                let (variables, weights) = row[..row.len() - 1]
                    .iter()
                    .enumerate()
                    .filter(|(_, w)| **w != 0)
                    .fold((vec![], vec![]), |(mut vars, mut weights), (i, weight)| {
                        vars.push(i);
                        weights.push(*weight);

                        (vars, weights)
                    });

                let value = row[row.len() - 1];

                Self::Polynomial {
                    variables,
                    weights,
                    value,
                }
            })
            .collect::<Vec<_>>()
    }
}

pub struct ConstraintSolver {
    /// Maximum range of values each variable can take
    domains: Vec<RangeInclusive<u32>>,
    /// Multi-variable constraints
    constraints: Vec<Constraint>,
}

impl ConstraintSolver {
    pub fn new(domains: Vec<RangeInclusive<u32>>, constraints: Vec<Constraint>) -> Self {
        Self {
            domains,
            constraints,
        }
    }

    /// Backtrack
    fn solve(
        &self,
        candidates: &mut Vec<u32>,
        constraints: &[Vec<Constraint>],
        mut max_presses: u32,
    ) -> u32 {
        for new_val in self.domains[candidates.len()].clone() {
            candidates.push(new_val);

            let objective = candidates.iter().sum::<u32>();

            // Only consider solutions which are less than our best current objective
            if objective < max_presses {
                // Check constraints for this level
                // Constraints for previous level are guaranteed to still hold
                let consistent = constraints[candidates.len() - 1]
                    .iter()
                    // .inspect(|_| self.constraint_evals += 1)
                    .all(|c| c.evaluate(candidates));

                // println!("{:?} -> {}", candidates, consistent);

                if consistent {
                    if candidates.len() == self.domains.len() {
                        println!("Solution! {:?}, presses: {}", candidates, objective);
                        max_presses = objective;
                    } else {
                        max_presses = self.solve(candidates, constraints, max_presses);
                    }
                }
            }

            candidates.pop();
        }

        max_presses
    }

    pub fn iter_solutions(&self) -> u32 {
        // Sort contraints into which can be evalauated at which level of recursion
        let mut constraints = vec![vec![]; self.domains.len()];
        for constraint in &self.constraints {
            let level = match constraint {
                Constraint::Polynomial { variables, .. } => *variables.iter().max().unwrap(),
            };
            constraints[level].push(constraint.clone());
        }

        self.solve(&mut vec![], &constraints, u32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::ConstraintSolver;

    use super::Constraint::*;
    #[test]
    fn test() {
        let domains = vec![0..=4, 0..=5, 0..=7, 0..=3, 0..=5, 0..=3];
        let constraints = vec![
            // Constraints from gaussian elimination
            Polynomial {
                variables: vec![0, 4, 5],
                weights: vec![1, 1, -1],
                value: 1,
            },
            Polynomial {
                variables: vec![1, 5],
                weights: vec![1, 1],
                value: 5,
            },
            Polynomial {
                variables: vec![2, 4],
                weights: vec![1, -1],
                value: 1,
            },
            Polynomial {
                variables: vec![3, 5],
                weights: vec![1, 1],
                value: 3,
            },
        ];

        let solver = ConstraintSolver::new(domains, constraints);
        solver.iter_solutions();
    }
}
