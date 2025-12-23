use std::{
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
    fn add_row(&mut self, columns: &[usize]) {
        // Expand columns if we need to
        let max_column = *columns.iter().max().unwrap();
        for _ in self.headers.len()..=max_column {
            self.add_column();
        }

        // Node indices that we're about to add
        let row_indices =
            (self.nodes.len()..(self.nodes.len() + columns.len())).collect::<Vec<_>>();

        for ((i, &col), &index) in columns.iter().enumerate().zip(&row_indices) {
            let head = self.headers[col];
            let tail = self.nodes[head].up;

            // We know how many we're adding with this row, so we can pre-select the indices for
            // L/R
            let left = row_indices[(row_indices.len() + i - 1).rem_euclid(row_indices.len())];
            let right = row_indices[(i + 1).rem_euclid(row_indices.len())];

            let node = Node {
                pos: (self.num_rows, col),
                up: tail,
                down: head,
                left,
                right,
            };

            self.nodes.push(node);

            // Update existing nodes along U/D
            self.nodes[head].up = index;
            self.nodes[tail].down = index;

            self.nodes_per_column[col] += 1;
        }

        self.num_rows += 1;
    }

    /// Sets which columns are required to be covered exactly once.
    /// Other columns will be covered at most once
    fn set_primary_columns(&mut self, columns: &[usize]) {
        self.primary_columns = columns.to_vec()
    }

    fn cover_column(&mut self, col: usize) {
        assert!(!self.covered_columns[col], "Column already covered!");

        // Unlink the column header from it's neighbours
        let column_header = self.headers[col];
        self.unlink_node(column_header, false);

        // Go through all rows that intersect with this column, and unlink them from their
        // neighbouring rows
        let mut column_node = self.nodes[column_header].down;
        while column_node != column_header {
            // println!(
            //     "Unlinking row starting with node: {} {:?}",
            //     column_node, self.nodes[column_node]
            // );
            // Unlink all nodes along this row from above/below
            // The intersecting row node is not snipped as it is used during the re-linking process
            // and it is excluded from search due to the column header being unlinked.
            let mut row_node = self.nodes[column_node].right;
            while row_node != column_node {
                self.unlink_node(row_node, true);
                self.nodes_per_column[self.nodes[row_node].pos.1] -= 1;
                row_node = self.nodes[row_node].right;
            }

            column_node = self.nodes[column_node].down;
        }

        self.covered_columns[col] = true;
    }

    /// Unlink a node along one axis
    fn unlink_node(&mut self, node: NodeIndex, vertical: bool) {
        // println!(
        //     "Unlinking node: {} {:?} - {}",
        //     node, self.nodes[node], vertical
        // );
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

    fn uncover_column(&mut self, col: usize) {
        assert!(self.covered_columns[col], "Column is not covered!");

        // Re-link the column header from it's neighbours
        let column_header = self.headers[col];
        self.relink_node(column_header, false);

        // Go through all rows that intersect with this column, and re-link them with their
        // neighbouring rows
        let mut column_node = self.nodes[column_header].up;
        while column_node != column_header {
            // Unlink all nodes along this row from above/below
            // The intersecting row node is not snipped as it is used during the re-linking process
            // and it is excluded from search due to the column header being unlinked.
            let mut row_node = self.nodes[column_node].left;
            while row_node != column_node {
                self.relink_node(row_node, true);
                self.nodes_per_column[self.nodes[row_node].pos.1] += 1;
                row_node = self.nodes[row_node].left;
            }

            column_node = self.nodes[column_node].up;
        }

        self.covered_columns[col] = false;
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

    fn walk_right(&self, start: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        successors(Some(start), move |prev| {
            let next = self.nodes[*prev].right;

            (start != next).then_some(next)
        })
    }

    fn walk_down(&self, start: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        successors(Some(start), move |prev| {
            let next = self.nodes[*prev].down;

            (start != next).then_some(next)
        })
    }

    fn walk_left(&self, start: NodeIndex) -> impl Iterator<Item = NodeIndex> {
        successors(Some(start), move |prev| {
            let next = self.nodes[*prev].left;

            (start != next).then_some(next)
        })
    }

    fn solve_recursive(&mut self, solution: &mut Vec<usize>) -> bool {
        // Select a column
        let Some(column_header) = self
            .walk_right(self.head)
            .skip(1)
            // Only select actions using primary columns
            .filter(|node| {
                let col = self.nodes[*node].pos.1;
                self.primary_columns.contains(&col)
            })
            .min_by_key(|node| {
                let col = self.nodes[*node].pos.1;
                self.nodes_per_column[col]
            })
        else {
            // All primary columns have been covered, so we have a solution
            return true;
        };

        if self.nodes_per_column[column_header] == 0 {
            // Dead end solution
            return false;
        }

        let column = self.nodes[column_header].pos.1;
        // println!("Selected column: {}", column);

        // Cover it, and conflicting rows
        self.cover_column(column);

        // Walk through possible choices which cover this condition
        let choices = self
            .walk_down(self.headers[column])
            .skip(1)
            .collect::<Vec<_>>();
        // println!("Choices: {:?}", choices);

        for row_node in choices {
            // println!("Trying row node {}: {:?}", row_node, self.nodes[row_node]);

            // For each choice, cover all other columns which this choice solves
            for col_node in self.walk_right(row_node).skip(1).collect::<Vec<_>>() {
                // println!(
                //     "Covering column node {}: {:?}",
                //     col_node, self.nodes[col_node]
                // );
                self.cover_column(self.nodes[col_node].pos.1);
            }

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

            for col_node in self.walk_left(row_node).skip(1).collect::<Vec<_>>() {
                self.uncover_column(self.nodes[col_node].pos.1);
            }
        }

        self.uncover_column(column);

        false
    }

    /// Solve the exact cover problem. Consumes the struct as it modifies internal state during the
    /// search
    fn solve(mut self) -> Option<Vec<usize>> {
        let mut solution = vec![];
        self.solve_recursive(&mut solution).then_some(solution)
    }
}

impl Display for DancingLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_nodes = self.nodes.iter().collect::<Vec<_>>();
        sorted_nodes.sort_by_key(|node| node.pos);

        let mut sorted_nodes = sorted_nodes.into_iter().peekable();

        for row in 0..self.num_rows {
            f.write_str(&format!("{:>5} ", row))?;

            let mut col = 0;
            while let Some(node) = sorted_nodes.next_if(|node| node.pos.0 == row) {
                while col < node.pos.1 {
                    f.write_char('.')?;
                    col += 1;
                }
                f.write_char('#')?;
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
            let piece_cols = pieces.iter().sum::<usize>();

            let mut piece_col = 0..piece_cols;

            // Create an incidence matrix - Columns are constraints (required & optional), rows are
            // choices to be made (piece x position)
            let incidence_matrix = shapes.iter().enumerate().flat_map(|(shape_index, shape)| {
                let board_choices = shape
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
                                    .map(|(i, j)| piece_cols + i * w + j)
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                // Assign one piece to each set of choices
                std::iter::repeat_n(board_choices, pieces[shape_index])
                    .zip(piece_col.by_ref())
                    .flat_map(|(mut choices, i)| {
                        choices.iter_mut().for_each(|row| {
                            row.push(i);
                        });

                        choices
                    })
                    .collect::<Vec<_>>()
            });

            let mut dl = DancingLinks::new();
            for row in incidence_matrix {
                dl.add_row(&row);
            }
            dl.set_primary_columns(&(0..piece_cols).collect::<Vec<_>>());

            dl.solve()
        })
        .enumerate()
        .inspect(|(i, solution)| {
            println!("Problem {i}: {:?}", solution);
        })
        .count();

    println!("{answer}");
}
