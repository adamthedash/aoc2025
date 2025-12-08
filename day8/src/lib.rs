use std::{cmp::Ordering, io::stdin, iter::Peekable};

pub fn parse_input() -> Vec<[u64; 3]> {
    stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split(',')
                .map(|val| val.parse().expect("Failed to parse int!"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Wrong amount of coords!")
        })
        .collect()
}

pub struct SortedInterleave<I1, I2, F, M>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    F: Fn(&I1::Item, &I2::Item) -> Ordering,
    M: Fn(&I1::Item) -> bool,
{
    left: Peekable<I1>,
    right: Peekable<I2>,
    cmp_func: F,
    /// Function which decides when it can skip the check for the 2nd iterator
    fail_func: M,
}

impl<I1, I2, F, M> SortedInterleave<I1, I2, F, M>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    F: Fn(&I1::Item, &I2::Item) -> Ordering,
    M: Fn(&I1::Item) -> bool,
{
    pub fn new(left: I1, right: I2, cmp_func: F, fail_func: M) -> Self {
        Self {
            left: left.peekable(),
            right: right.peekable(),
            cmp_func,
            fail_func,
        }
    }
}

impl<I1, I2, F, M> Iterator for SortedInterleave<I1, I2, F, M>
where
    I1: Iterator,
    I2: Iterator<Item = I1::Item>,
    F: Fn(&I1::Item, &I2::Item) -> Ordering,
    M: Fn(&I1::Item) -> bool,
{
    type Item = I1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.left.peek();
        let right = if left.is_none_or(|x| (self.fail_func)(x)) {
            // No items left, or it's possible that a better item is in right
            self.right.peek()
        } else {
            // Left is guaranteed to be better
            None
        };

        match (left, right) {
            (None, None) => None,
            (None, Some(_)) => self.right.next(),
            (Some(_), None) => self.left.next(),
            (Some(x), Some(y)) => match (self.cmp_func)(x, y) {
                Ordering::Less => self.left.next(),
                Ordering::Equal => self.left.next(),
                Ordering::Greater => self.right.next(),
            },
        }
    }
}

/// Represents a single node in a KDTree
#[derive(Debug)]
enum Node {
    Leaf(usize),
    Split {
        dim: usize,
        value: u64,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    /// Build a sub-tree with the given set of nodes
    pub fn build(indices: &mut [usize], points_lut: &[[u64; 3]], dim: usize) -> Self {
        if indices.len() == 1 {
            return Self::Leaf(indices[0]);
        }

        // Split on median
        indices.sort_unstable_by_key(|index| points_lut[*index][dim]);
        let median = points_lut[indices[indices.len() / 2]][dim];
        let median_index = indices.partition_point(|index| points_lut[*index][dim] < median);

        let (left, right) = indices.split_at_mut(median_index);

        Self::Split {
            dim,
            value: median,
            left: Box::new(Self::build(left, points_lut, (dim + 1) % 3)),
            right: Box::new(Self::build(right, points_lut, (dim + 1) % 3)),
        }
    }

    /// Returns the single nearest neighbour to the test point
    pub fn nearest_neighbour(&self, point: &[u64; 3], points_lut: &[[u64; 3]]) -> (usize, u64) {
        use Node::*;
        match self {
            Leaf(index) => (*index, dist2(point, &points_lut[*index])),
            Split {
                dim,
                value: split_point,
                left,
                right,
            } => {
                // Look at the side which has the best chance of having the nearest first
                let search_order = if point[*dim] < *split_point {
                    [left, right]
                } else {
                    [right, left]
                };

                // Try first side
                let first = search_order[0].nearest_neighbour(point, points_lut);

                // If the test point is closer to the split point than the closest point from the
                // first arm, then there's a chance the real closest is in the other one, so
                // search there aswell
                if first.1 > split_point.abs_diff(point[*dim]) {
                    let second = search_order[1].nearest_neighbour(point, points_lut);

                    if first.1 < second.1 { first } else { second }
                } else {
                    first
                }
            }
        }
    }

    /// Returns all neighbours in sorted order from nearest to farthest
    pub fn nearest_neighbours<'a>(
        &'a self,
        point: &'a [u64; 3],
        points_lut: &'a [[u64; 3]],
    ) -> Box<dyn Iterator<Item = (usize, u64)> + 'a> {
        use Node::*;
        match self {
            Leaf(index) => Box::new(std::iter::once((*index, dist2(point, &points_lut[*index])))),
            Split {
                dim,
                value: split_point,
                left,
                right,
            } => {
                // Look at the side which has the best chance of having the nearest first
                let search_order = if point[*dim] < *split_point {
                    [left, right]
                } else {
                    [right, left]
                };

                // Get closest from first side (lazy init)
                // https://stackoverflow.com/a/49456073
                let first = [()]
                    .into_iter()
                    .flat_map(move |_| search_order[0].nearest_neighbours(point, points_lut));

                // Get cloestest from 2nd side (Lazy init)
                let second = [()]
                    .into_iter()
                    .flat_map(move |_| search_order[1].nearest_neighbours(point, points_lut));

                let nearest = SortedInterleave::new(
                    first,
                    second,
                    |(_, d1), (_, d2)| d1.cmp(d2),
                    // Check whether 2nd side needs to be searched
                    |(_, d1)| *d1 > split_point.abs_diff(point[*dim]).pow(2),
                );

                Box::new(nearest)
            }
        }
    }
}

/// Distance squared
pub fn dist2(a: &[u64; 3], b: &[u64; 3]) -> u64 {
    a.iter().zip(b).map(|(a, b)| a.abs_diff(*b).pow(2)).sum()
}

#[derive(Debug)]
pub struct KDTree<'a> {
    /// Reference to contiguous point storage
    points: &'a [[u64; 3]],
    /// Tree structure
    root: Node,
}

impl<'a> KDTree<'a> {
    /// Build a new KDTree with the given set of points
    pub fn build(points: &'a [[u64; 3]]) -> Self {
        let mut indices = (0..points.len()).collect::<Vec<_>>();
        let root = Node::build(&mut indices, points, 0);

        Self { points, root }
    }

    /// Iterate over nearest neighbours from closest to furthest.
    pub fn nearest_neighbours(
        &'a self,
        point: &'a [u64; 3],
    ) -> impl Iterator<Item = (usize, u64)> + 'a {
        self.root.nearest_neighbours(point, self.points)
    }

    /// Find the single nearest neighbour
    pub fn nearest_neighbour(&self, point: &[u64; 3]) -> (usize, u64, &[u64; 3]) {
        let (index, dist2) = self.root.nearest_neighbour(point, self.points);

        (index, dist2, self.get(index))
    }

    /// Get the corresponding point given an index
    pub fn get(&self, index: usize) -> &[u64; 3] {
        &self.points[index]
    }
}
