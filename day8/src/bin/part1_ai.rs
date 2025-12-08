use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        let mut result: Vec<_> = sizes.values().copied().collect();
        result.sort_by(|a, b| b.cmp(a));
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<Point> = stdin
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();

    let n = points.len();

    let mut edges = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist_sq = points[i].distance_squared(&points[j]);
            edges.push((dist_sq, i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut uf = UnionFind::new(n);

    let connections_to_make = 1000.min(edges.len());

    for i in 0..connections_to_make {
        uf.union(edges[i].1, edges[i].2);
    }

    let circuit_sizes = uf.get_circuit_sizes();

    let size1 = circuit_sizes.get(0).copied().unwrap_or(1);
    let size2 = circuit_sizes.get(1).copied().unwrap_or(1);
    let size3 = circuit_sizes.get(2).copied().unwrap_or(1);
    let result = size1 * size2 * size3;
    println!("{}", result);
}
