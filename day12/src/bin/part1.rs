use std::{
    collections::HashMap,
    fmt::{Display, Write},
    iter::successors,
};

use day12::parse_input;

type NodeIndex = usize;

/// Represents a 1 in the incidence matrix
#[derive(Debug)]
struct Node {
    /// Position of this 1 in the matrix
    pos: (usize, usize),
    /// How many instances of this node are left
    remaining: usize,
    /// Pointers to adjacent nodes
    left: NodeIndex,
    right: NodeIndex,
    up: NodeIndex,
    down: NodeIndex,
}

struct DancingLinks {
    /// Dense storage of all nodes
    nodes: Vec<Node>,
    /// Pointer to top-left node
    head: NodeIndex,
    /// Pointers to "Control row" nodes. Doesn't store real data
    headers: Vec<NodeIndex>,
    /// Masks of rows/columns which have been covered
    covered_columns: Vec<bool>,
    covered_rows: Vec<bool>,
    /// Count of nodes in each column
    nodes_per_column: Vec<usize>,
    /// Number of possible choices that can be made
    num_rows: usize,
    /// Primary columns
    primary_columns: Vec<usize>,
}

impl DancingLinks {
    /// Create a new Dancing Links matrix initialised with a single head node
    fn new() -> Self {
        let head = Node {
            // Header doesn't lie in the table
            pos: (usize::MAX, usize::MAX),
            remaining: 1,
            // First node always points to itself
            left: 0,
            right: 0,
            up: 0,
            down: 0,
        };

        Self {
            nodes: vec![head],
            head: 0,
            headers: vec![],
            covered_columns: vec![],
            covered_rows: vec![],
            nodes_per_column: vec![],
            num_rows: 0,
            primary_columns: vec![],
        }
    }

    /// Add a new constraint column
    fn add_column(&mut self) {
        let index = self.nodes.len();
        let tail = self.nodes[self.head].left;
        let node = Node {
            // Not a real position here
            pos: (usize::MAX, self.headers.len()),
            // Headers only have 1 instance, as they are not decremented
            remaining: 1,
            // New column placed after last / before start node
            left: tail,
            right: self.head,
            // New column, so there'll be no vertical entries
            up: index,
            down: index,
        };

        self.nodes.push(node);

        // Update existing nodes
        self.nodes[self.head].left = index;
        self.nodes[tail].right = index;

        // Add to metadata
        self.headers.push(index);
        self.covered_columns.push(false);
        self.nodes_per_column.push(0);
    }

    /// Add a new row (choice) to the matrix using sparse column indices
    fn add_row(&mut self, columns: &HashMap<usize, usize>) {
        // Expand columns if we need to
        let max_column = *columns.keys().max().unwrap();
        for _ in self.headers.len()..=max_column {
            self.add_column();
        }

        // Node indices that we're about to add
        let row_indices =
            (self.nodes.len()..(self.nodes.len() + columns.len())).collect::<Vec<_>>();

        for ((i, (&col, &count)), &index) in columns.iter().enumerate().zip(&row_indices) {
            let head = self.headers[col];
            let tail = self.nodes[head].up;

            // We know how many we're adding with this row, so we can pre-select the indices for
            // L/R
            let left = row_indices[(row_indices.len() + i - 1).rem_euclid(row_indices.len())];
            let right = row_indices[(i + 1).rem_euclid(row_indices.len())];

            let node = Node {
                pos: (self.num_rows, col),
                remaining: count,
                up: tail,
                down: head,
                left,
                right,
            };

            self.nodes.push(node);

            // Update existing nodes along U/D
            self.nodes[head].up = index;
            self.nodes[tail].down = index;

            self.nodes_per_column[col] += count;
        }

        self.num_rows += 1;
        self.covered_rows.push(false);
    }

    /// Sets which columns are required to be covered exactly once.
    /// Other columns will be covered at most once
    fn set_primary_columns(&mut self, columns: &[usize]) {
        self.primary_columns = columns.to_vec()
    }

    /// Decrement the remaining counter by 1 along a column
    fn decr_column(&mut self, col: usize) {
        let column_header = self.headers[col];

        for col_node in self.walk_down(column_header).skip(1).collect::<Vec<_>>() {
            self.nodes[col_node].remaining -= 1;
            self.nodes_per_column[col] -= 1;
        }
    }

    /// Increment the remaining counter by 1 along a column
    fn incr_column(&mut self, col: usize) {
        let column_header = self.headers[col];

        for col_node in self.walk_down(column_header).skip(1).collect::<Vec<_>>() {
            self.nodes[col_node].remaining += 1;
            self.nodes_per_column[col] += 1;
        }
    }

    /// Unlink a node along one axis
    fn unlink_node(&mut self, node: NodeIndex, vertical: bool) {
        if vertical {
            let up = self.nodes[node].up;
            let down = self.nodes[node].down;
            self.nodes[up].down = down;
            self.nodes[down].up = up;
        } else {
            let left = self.nodes[node].left;
            let right = self.nodes[node].right;
            self.nodes[left].right = right;
            self.nodes[right].left = left;
        }
    }

    /// Re-link a node along one axis
    fn relink_node(&mut self, node: NodeIndex, vertical: bool) {
        if vertical {
            let up = self.nodes[node].up;
            let down = self.nodes[node].down;
            self.nodes[up].down = node;
            self.nodes[down].up = node;
        } else {
            let left = self.nodes[node].left;
            let right = self.nodes[node].right;
            self.nodes[left].right = node;
            self.nodes[right].left = node;
        }
    }

    /// Walk from a node to others connected to the right, non-repeating
    fn walk_right(&self, start: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        successors(Some(start), move |prev| {
            let next = self.nodes[*prev].right;

            (start != next).then_some(next)
        })
    }

    /// Walk from a node to others connected downwards, non-repeating
    fn walk_down(&self, start: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        successors(Some(start), move |prev| {
            let next = self.nodes[*prev].down;

            (start != next).then_some(next)
        })
    }

    /// Add a row to the solution, updating the matrix and removing conflicting choices
    fn choose(&mut self, row_node: NodeIndex) {
        self.covered_rows[self.nodes[row_node].pos.0] = true;

        // Remove this choice node
        self.nodes_per_column[self.nodes[row_node].pos.1] -= self.nodes[row_node].remaining;
        self.unlink_node(row_node, true);

        // Decrement & unlink the nodes along this row, but on other columns
        for col_node in self.walk_right(row_node).skip(1).collect::<Vec<_>>() {
            self.nodes[col_node].remaining -= 1;
            self.nodes_per_column[self.nodes[col_node].pos.1] -= 1;
            self.unlink_node(col_node, true);
        }

        // Cover columns which conflict with this choice
        for col_node in self.walk_right(row_node).skip(1).collect::<Vec<_>>() {
            let column2 = self.nodes[col_node].pos.1;
            self.decr_column(column2);
            if self.nodes_per_column[column2] == 0 {
                self.covered_columns[column2] = true;
            }

            // Remove rows which conflict with newly covered columns
            for row_node2 in self
                .walk_down(self.headers[column2])
                .skip(1)
                .collect::<Vec<_>>()
            {
                self.covered_rows[self.nodes[row_node2].pos.0] = true;

                for col_node2 in self.walk_right(row_node2).skip(1).collect::<Vec<_>>() {
                    let column3 = self.nodes[col_node2].pos.1;
                    self.nodes_per_column[column3] -= self.nodes[col_node2].remaining;
                    if self.nodes_per_column[column3] == 0 {
                        self.covered_columns[column3] = true;
                    }

                    self.unlink_node(col_node2, true);
                }
            }
        }
    }

    // Un-do the coverings applied when adding a row to the solution
    fn unchoose(&mut self, row_node: NodeIndex) {
        self.covered_rows[self.nodes[row_node].pos.0] = false;

        // Re-link this choice node
        self.nodes_per_column[self.nodes[row_node].pos.1] += self.nodes[row_node].remaining;
        self.relink_node(row_node, true);

        // Increment & re-link the nodes along this row, but on other columns
        for col_node in self.walk_right(row_node).skip(1).collect::<Vec<_>>() {
            self.nodes[col_node].remaining += 1;
            self.nodes_per_column[self.nodes[col_node].pos.1] += 1;
            self.relink_node(col_node, true);
        }

        // Uncover columns which conflict with this choice
        for col_node in self.walk_right(row_node).skip(1).collect::<Vec<_>>() {
            let column2 = self.nodes[col_node].pos.1;
            if self.nodes_per_column[column2] == 0 {
                self.covered_columns[column2] = false;
            }
            self.incr_column(column2);

            // Re-add rows which conflict with uncovered columns
            for row_node2 in self
                .walk_down(self.headers[column2])
                .skip(1)
                .collect::<Vec<_>>()
            {
                self.covered_rows[self.nodes[row_node2].pos.0] = false;

                for col_node2 in self.walk_right(row_node2).skip(1).collect::<Vec<_>>() {
                    let column3 = self.nodes[col_node2].pos.1;

                    if self.nodes_per_column[column3] == 0 {
                        self.covered_columns[column3] = false;
                    }
                    self.nodes_per_column[column3] += self.nodes[col_node2].remaining;

                    self.relink_node(col_node2, true);
                }
            }
        }
    }

    fn solve_recursive(&mut self, solution: &mut Vec<usize>) -> bool {
        // Select a column
        let Some(column_header) = self
            .walk_right(self.head)
            .skip(1)
            // Only select actions using primary columns & only non-covered ones
            .filter(|node| {
                let col = self.nodes[*node].pos.1;
                self.primary_columns.contains(&col) && !self.covered_columns[col]
            })
            .min_by_key(|node| {
                let col = self.nodes[*node].pos.1;
                self.nodes_per_column[col]
            })
        else {
            // All primary columns have been covered, so we have a solution
            return true;
        };

        let column = self.nodes[column_header].pos.1;

        if self.nodes_per_column[column] == 0 {
            // Dead end solution
            return false;
        }

        // Sub 1 from the chosen column, since that's the constraint we're solving
        self.decr_column(column);
        if self.nodes_per_column[column] == 0 {
            self.covered_columns[column] = true;
            self.unlink_node(column_header, false);
        }

        // Walk through possible choices which cover this condition
        for row_node in self
            .walk_down(self.headers[column])
            .skip(1)
            .collect::<Vec<_>>()
        {
            self.choose(row_node);

            // Add it to the partial solution
            let row = self.nodes[row_node].pos.0;
            solution.push(row);
            // println!("Trying: {:?}", solution);

            // Recurse
            if self.solve_recursive(solution) {
                // One of the sub-solutions has been successful, so return out
                return true;
            }

            // Un-do the coverings
            solution.pop();

            self.unchoose(row_node);
        }

        // Sub 1 from the chosen column, since that's the constraint we're solving
        if self.nodes_per_column[column] == 0 {
            self.covered_columns[column] = false;
            self.relink_node(column_header, false);
        }
        self.incr_column(column);

        false
    }

    /// Solve the exact cover problem. Consumes the struct as it modifies internal state during the
    /// search
    fn solve(mut self) -> Option<Vec<usize>> {
        // First need to mark any columns with no options as already covered
        for col in self
            .nodes_per_column
            .iter()
            .enumerate()
            .filter_map(|(col, count)| (*count == 0).then_some(col))
            .collect::<Vec<_>>()
        {
            self.covered_columns[col] = true;
            self.unlink_node(self.headers[col], false);
        }

        let mut solution = vec![];
        self.solve_recursive(&mut solution).then_some(solution)
    }
}

impl Display for DancingLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_nodes = self.nodes.iter().collect::<Vec<_>>();
        sorted_nodes.sort_by_key(|node| node.pos);

        let mut sorted_nodes = sorted_nodes.into_iter().peekable();

        // Coverage
        f.write_str(&format!("{:>5} ", ""))?;
        for covered in &self.covered_columns {
            if *covered {
                f.write_char('X')?;
            } else {
                f.write_char(' ')?;
            }
        }
        f.write_char('\n')?;

        for (row, covered) in self.covered_rows.iter().enumerate() {
            f.write_str(&format!("{:>5}", row))?;
            if *covered {
                f.write_char('X')?;
            } else {
                f.write_char(' ')?;
            }

            let mut col = 0;
            while let Some(node) = sorted_nodes.next_if(|node| node.pos.0 == row) {
                while col < node.pos.1 {
                    f.write_char('.')?;
                    col += 1;
                }
                f.write_str(&format!("{}", node.remaining))?;
                // f.write_char('#')?;
                col += 1;
            }
            while col < self.headers.len() {
                f.write_char('.')?;
                col += 1;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn main() {
    let (shapes, problems) = parse_input();

    let shape_coverage = shapes
        .iter()
        .map(|s| s.sparse_mask().count())
        .collect::<Vec<_>>();

    let answer = problems
        .map(|(h, w, pieces)| {
            // Quick check - if there's enough area to hold all the presents
            let area_needed = pieces
                .iter()
                .zip(&shape_coverage)
                .map(|(n, a)| n * a)
                .sum::<usize>();
            if area_needed > h * w {
                // Not enough area
                return None;
            }

            // In-depth check - Exact cover with Algorithm X / Dancing Links
            let piece_cols = pieces.len();

            // Create an incidence matrix - Columns are constraints (required & optional), rows are
            // choices to be made (piece x position)
            let incidence_matrix = shapes
                .iter()
                .enumerate()
                .filter(|(shape_index, _)| pieces[*shape_index] > 0)
                .flat_map(|(shape_index, shape)| {
                    let mut board_choices = shape
                        .clone()
                        .enumerate_orientations()
                        .flat_map(|shape| {
                            // Shape translations
                            let offsets = (0..h - shape.height() + 1).flat_map(|offset_i| {
                                (0..w - shape.width() + 1).map(move |offset_j| (offset_i, offset_j))
                            });

                            let sparse_shape = shape.sparse_mask().collect::<Vec<_>>();

                            // Create sparse shape at each position
                            offsets
                                .map(|(oi, oj)| {
                                    sparse_shape
                                        .iter()
                                        // Apply translation
                                        .map(|(i, j)| (i + oi, j + oj))
                                        // Convert to incidence matrix indices
                                        .map(|(i, j)| (piece_cols + i * w + j, 1))
                                        .collect::<HashMap<_, _>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();

                    // Assign board pieces
                    board_choices.iter_mut().for_each(|row| {
                        row.insert(shape_index, pieces[shape_index]);
                    });

                    board_choices
                });

            let mut dl = DancingLinks::new();
            for row in incidence_matrix {
                dl.add_row(&row);
            }
            dl.set_primary_columns(&(0..piece_cols).collect::<Vec<_>>());
            // println!("{}", dl);

            dl.solve()
        })
        .enumerate()
        .inspect(|(i, solution)| {
            println!("Problem {i}: {:?}", solution);
        })
        .filter(|(_, solution)| solution.is_some())
        .count();

    println!("{answer}");
}
