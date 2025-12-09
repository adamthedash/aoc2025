use day9::parse_input;
use svg::{
    Document,
    node::element::{Path, path::Data},
};

fn draw_svg(tiles: &[[u64; 2]]) {
    let max_xy = tiles.iter().fold([0; 2], |mut acc, p| {
        acc.iter_mut().zip(p).for_each(|(acc, p)| {
            if p > acc {
                *acc = *p;
            }
        });
        acc
    });

    // Draw paths
    let tiles = tiles
        .iter()
        .map(|t| (t[0] as f32 / 100., t[1] as f32 / 100.))
        .collect::<Vec<_>>();
    let data = Data::new().move_to(*tiles.last().unwrap());
    let data = tiles.iter().fold(data, |data, tile| data.line_to(*tile));

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new()
        // .set("viewBox", (0, 0, max_xy[1] + 1, max_xy[0] + 1))
        .set("viewBox", (0, 0, 1000, 1000))
        .add(path);

    svg::save("test.svg", &document).unwrap();
}

struct AABB {
    start: [u64; 2],
    end: [u64; 2],
}

impl AABB {
    fn new(p1: &[u64; 2], p2: &[u64; 2]) -> Self {
        let start = p1
            .iter()
            .zip(p2)
            .map(|(p1, p2)| *(p1.min(p2)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let end = p1
            .iter()
            .zip(p2)
            .map(|(p1, p2)| *(p1.max(p2)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { start, end }
    }

    /// A line intersects the AABB if it goes through the middle, not flush with edges
    fn intersects_line(&self, p0: &[u64; 2], p1: &[u64; 2]) -> bool {
        // Dimension that line runs along
        let through_dim = p0.iter().zip(p1).position(|(p0, p1)| p0 != p1).unwrap();
        let within_dim = 1 - through_dim;

        // Sort points
        let (p0, p1) = if p0[through_dim] > p1[through_dim] {
            (p1, p0)
        } else {
            (p0, p1)
        };

        p0[through_dim] < self.end[through_dim]
            && p1[through_dim] > self.start[through_dim]
            && self.start[within_dim] < p0[within_dim]
            && p0[within_dim] < self.end[within_dim]
    }

    fn area(&self) -> u64 {
        self.end
            .iter()
            .zip(&self.start)
            .map(|(e, s)| e - s + 1)
            .product()
    }
}

fn main() {
    let tiles = parse_input();
    // draw_svg(&tiles);

    // Exhaustive pairs of points
    let pairs = tiles
        .iter()
        .enumerate()
        .take(tiles.len() - 1)
        .flat_map(|(i, t1)| tiles.iter().skip(i + 1).map(move |t2| (t1, t2)));

    // Perimeter around enclosed polygon
    let enclosed_tiles = tiles
        .iter()
        .chain(std::iter::once(&tiles[0]))
        .collect::<Vec<_>>();
    let lines = enclosed_tiles.windows(2).collect::<Vec<_>>();

    // Try all possible AABB's that are within the designated region
    let aabbs = pairs.map(|(t1, t2)| AABB::new(t1, t2)).filter(|aabb| {
        !lines
            .iter()
            .any(|line| aabb.intersects_line(line[0], line[1]))
    });

    let max_area = aabbs.map(|aabb| aabb.area()).max().unwrap();
    println!("{max_area}");
}
