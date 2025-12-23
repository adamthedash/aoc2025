use std::{
    collections::HashSet,
    fmt::{Display, Write},
    hash::{BuildHasher, Hash},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Shape {
    pub mask: Vec<Vec<bool>>,
    pub worth: Vec<usize>,
}

impl Shape {
    /// Rotate a shape clockwise 90 degrees
    pub fn rotate(&self) -> Self {
        let mut out = vec![vec![false; self.mask.len()]; self.mask.len()];

        for (i, row) in self.mask.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                out[j][self.mask.len() - i - 1] = *x;
            }
        }

        Self {
            mask: out,
            worth: self.worth.clone(),
        }
    }

    /// Flip a shape vertically
    pub fn flip(&self) -> Self {
        let out = self.mask.iter().rev().cloned().collect();

        Self {
            mask: out,
            worth: self.worth.clone(),
        }
    }

    /// Generate all unique rotations / flips of a shape, including the identity
    pub fn enumerate_orientations(self) -> impl Iterator<Item = Self> {
        let shapes = std::iter::successors(Some((self, 0)), |(shape, i)| {
            let shape = if *i == 3 {
                shape.flip()
            } else {
                shape.rotate()
            };

            Some((shape, i + 1))
        })
        .map(|(shape, _)| shape)
        .take(8);

        remove_duplicates(shapes)
    }

    /// Attempt to build a combined shape at the given offset
    pub fn tesselate(&self, other: &Self, offset: (isize, isize)) -> Option<Self> {
        let (io, jo) = offset;

        let h1 = self.mask.len() as isize;
        let h2 = other.mask.len() as isize;
        let w1 = self.mask[0].len() as isize;
        let w2 = other.mask[0].len() as isize;

        // Region where we sample from self.mask
        let min_i1 = (-io).max(0);
        let max_i1 = h1 + (-io).max(0);
        let min_j1 = (-jo).max(0);
        let max_j1 = w1 + (-jo).max(0);
        let i1 = min_i1..max_i1;
        let j1 = min_j1..max_j1;

        // Region where we sample from shape2
        let min_i2 = io.max(0);
        let max_i2 = h2 + io.max(0);
        let min_j2 = jo.max(0);
        let max_j2 = w2 + jo.max(0);
        let i2 = min_i2..max_i2;
        let j2 = min_j2..max_j2;

        // Output shape
        let h_out = (h1 - io.min(0)).max(h2 + io.max(0));
        let w_out = (w1 - jo.min(0)).max(w2 + jo.max(0));
        let mut out = vec![vec![false; w_out as usize]; h_out as usize];

        // Build the tesselated shape
        for i in 0..h_out {
            for j in 0..w_out {
                let in1 = i1.contains(&i) && j1.contains(&j);
                let in2 = i2.contains(&i) && j2.contains(&j);
                match (in1, in2) {
                    (true, true) => {
                        // In both, so need to check for conflicts
                        let sample1 = self.mask[(i - min_i1) as usize][(j - min_j1) as usize];
                        let sample2 = other.mask[(i - min_i2) as usize][(j - min_j2) as usize];

                        if sample1 && sample2 {
                            // Conflict! Break of this tesselation
                            return None;
                        } else {
                            out[i as usize][j as usize] = sample1 || sample2;
                        }
                    }
                    (true, false) => {
                        // Sample from self.mask without checks
                        out[i as usize][j as usize] =
                            self.mask[(i - min_i1) as usize][(j - min_j1) as usize];
                    }
                    (false, true) => {
                        // Sample from shape2 without checks
                        out[i as usize][j as usize] =
                            other.mask[(i - min_i2) as usize][(j - min_j2) as usize];
                    }
                    (false, false) => {
                        // Outside of both shapes, so do nothing
                    }
                }
            }
        }

        // Combine shape counts
        let worth = self
            .worth
            .iter()
            .zip(&other.worth)
            .map(|(a, b)| a + b)
            .collect();

        Some(Self { mask: out, worth })
    }

    /// Enumerate all possible tesselations of two shapes
    /// A tesselation has at least 1 tile of overlap
    pub fn enumerate_tesselations(&self, other: &Self) -> impl Iterator<Item = Shape> {
        let h1 = self.mask.len() as isize;
        let h2 = other.mask.len() as isize;
        let w1 = self.mask[0].len() as isize;
        let w2 = other.mask[0].len() as isize;

        let offsets = ((1 - h2)..=(h1 - 1)).flat_map(move |y_offset| {
            ((1 - w2)..=(w1 - 1)).map(move |x_offset| (y_offset, x_offset))
        });

        offsets.flat_map(|offsets| self.tesselate(other, offsets))
    }

    pub fn height(&self) -> usize {
        self.mask.len()
    }

    pub fn width(&self) -> usize {
        self.mask[0].len()
    }

    pub fn area(&self) -> usize {
        self.height() * self.width()
    }

    pub fn sparse_mask(&self) -> impl Iterator<Item = (usize, usize)> {
        self.mask.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x)
                .map(move |(j, _)| (i, j))
        })
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.mask.iter().enumerate() {
            for x in row {
                if *x {
                    // f.write_char('#')?;
                    f.write_str("[]")?;
                } else {
                    // f.write_char('.')?;
                    f.write_str("  ")?;
                }
            }
            if i < self.height() - 1 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

pub fn remove_duplicates(shapes: impl Iterator<Item = Shape>) -> impl Iterator<Item = Shape> {
    shapes
        .scan(HashSet::new(), |seen, shape| {
            let hash = seen.hasher().hash_one(&shape);

            let shape = if seen.contains(&hash) {
                None
            } else {
                seen.insert(hash);
                Some(shape)
            };

            Some(shape)
        })
        .flatten()
}

// impl Hash for Shape {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         for row in &self.mask {
//             let compressed = row.iter().fold(0, |acc, x| (acc | *x as u64) << 1);
//             compressed.hash(state);
//         }
//         // self.mask.hash(state);
//         self.worth.hash(state);
//     }
// }
