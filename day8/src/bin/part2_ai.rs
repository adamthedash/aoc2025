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
    num_circuits: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_circuits: n,
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

        self.num_circuits -= 1;
        true
    }

    fn is_single_circuit(&self) -> bool {
        self.num_circuits == 1
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

    let mut last_connection = (0, 0);

    for edge in edges {
        let (_, i, j) = edge;
        if uf.union(i, j) {
            last_connection = (i, j);
            if uf.is_single_circuit() {
                break;
            }
        }
    }

    let result = points[last_connection.0].x * points[last_connection.1].x;
    println!("{}", result);
}
