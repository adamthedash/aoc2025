pub struct GaussianElimination {
    rows: Vec<Vec<i64>>,
    /// Mapping for variables after column shuffling
    columns: Vec<usize>,
}

impl GaussianElimination {
    fn new(rows: Vec<Vec<i64>>) -> Self {
        Self {
            columns: (0..rows[0].len() - 1).collect(),
            rows,
        }
    }

    /// Divide a row by a constant
    fn div_scalar(&mut self, row: usize, div: i64) {
        assert_ne!(div, 0);
        self.rows[row].iter_mut().for_each(|x| {
            assert!(x.checked_rem(div).is_some_and(|rem| rem == 0));
            *x /= div
        });
    }

    /// Swap two rows
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.rows.swap(row1, row2);
    }

    /// Move column to a new location, shifting others appropriately
    fn shift_col(&mut self, src_col: usize, dst_col: usize) {
        assert!(src_col < self.columns.len() && dst_col < self.columns.len());

        let element = self.columns.remove(src_col);
        self.columns.insert(dst_col, element);

        self.rows.iter_mut().for_each(|row| {
            let element = row.remove(src_col);
            row.insert(dst_col, element);
        });
    }

    /// Add a multiple of one row to another
    fn add_row(&mut self, src_row: usize, dst_row: usize, mul: i64) {
        let [src_row, dst_row] = self
            .rows
            .get_disjoint_mut([src_row, dst_row])
            .expect("Rows much be disjoint");

        src_row.iter().zip(dst_row).for_each(|(s, d)| {
            *d += *s * mul;
        });
    }

    /// Subtract a row from all other rows, zeroing them out
    fn sub_rows(&mut self, column: usize, sub_row: usize) {
        // Sub anchor row from the rest
        for dst_row in (0..self.rows.len()).filter(|i| *i != sub_row) {
            let mul = -self.rows[dst_row][column];
            self.add_row(sub_row, dst_row, mul);
        }
    }

    /// Find the first row that has a 1 in the given column
    fn find_one_row(&self, column: usize, start_row: usize) -> Option<usize> {
        self.rows[start_row..]
            .iter()
            .position(|row| row[column] == 1)
            .map(|i| i + start_row)
    }

    /// Find the first row that is evenly divisible by the number in the given column
    fn find_divisible_row(&self, column: usize, start_row: usize) -> Option<usize> {
        self.rows[start_row..]
            .iter()
            .position(|row| {
                row.iter()
                    .all(|x| x.checked_rem(row[column]).is_some_and(|rem| rem == 0))
            })
            .map(|i| i + start_row)
    }

    /// Check if all of the rows are zero in this column
    fn check_all_zeros(&self, column: usize, start_row: usize) -> bool {
        self.rows[start_row..].iter().all(|row| row[column] == 0)
    }

    /// Simplify the contained matrix as much as possible
    fn solve(&mut self) {
        let mut non_reducible_cols = vec![];
        let mut column = 0;
        for row in 0..self.rows.len() {
            while column < self.rows[0].len() - 1 {
                // Find a row starting with 1
                if let Some(one_row) = self.find_one_row(column, row) {
                    // println!("Found 1-row: {:?} {:?}", one_row, self.rows[one_row]);
                    // Move it to the top
                    self.swap_rows(one_row, row);

                    // Subtract
                    self.sub_rows(column, row);
                } else if let Some(div_row) = self.find_divisible_row(column, row) {
                    // Try find a row that we can normalise to become a row starting with 1

                    // Move it to the top
                    self.swap_rows(div_row, row);

                    // Normalise it
                    self.div_scalar(row, self.rows[row][column]);

                    // Subtract
                    self.sub_rows(column, row);
                } else {
                    // We weren't able to find a row to operate on
                    if self.check_all_zeros(column, row) {
                        // The variable in this column isn't free for the rest of the rows, so check
                        // the next column

                        column += 1;
                        continue;
                    } else {
                        // There are rows to play with, but they're not integer-divisible
                        // println!("Non-reducible column!: {column}");
                        non_reducible_cols.push(column);
                        column += 1;
                        continue;
                    }
                }

                column += 1;
                break;
            }
        }

        // Swap non-reducible columns to the back
        non_reducible_cols
            .iter()
            .rev()
            .zip(0..)
            .for_each(|(src_col, i)| {
                let dst_col = self.columns.len() - 1 - i;
                // println!("Moving col: {} -> {}", src_col, dst_col);
                self.shift_col(*src_col, dst_col);
            });
    }

    /// Simplify a set of constraints
    pub fn simplify(rows: Vec<Vec<i64>>) -> (Vec<usize>, Vec<Vec<i64>>) {
        let mut ge = Self::new(rows);

        ge.solve();

        // Trim off any reduced constraints
        let rows = ge
            .rows
            .into_iter()
            .filter(|row| !row.iter().all(|x| *x == 0))
            .collect::<Vec<_>>();

        (ge.columns, rows)
    }
}
