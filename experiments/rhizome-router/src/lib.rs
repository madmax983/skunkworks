use std::collections::BinaryHeap;
use std::cmp::Ordering;
use ordered_float::NotNan;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub resistance: f32, // The "soil resistance"
    pub dist: f32,       // Current best distance from source
    pub visited: bool,   // Is this part of the finalized root system?
    pub parent: Option<(usize, usize)>,
    pub is_taproot: bool,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                // Initialize with some random noise for organic feel
                // Base resistance is high (soil)
                let noise = rng.gen_range(5.0..10.0);
                row.push(Cell {
                    x,
                    y,
                    resistance: noise,
                    dist: f32::INFINITY,
                    visited: false,
                    parent: None,
                    is_taproot: false,
                });
            }
            cells.push(row);
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn add_nutrient(&mut self, center_x: usize, center_y: usize, radius: f32, strength: f32) {
        let r_sq = radius * radius;
        // Optimization: only iterate bounding box
        let start_x = (center_x as f32 - radius).max(0.0) as usize;
        let end_x = (center_x as f32 + radius).min(self.width as f32) as usize;
        let start_y = (center_y as f32 - radius).max(0.0) as usize;
        let end_y = (center_y as f32 + radius).min(self.height as f32) as usize;

        for y in start_y..end_y {
            if y >= self.height { break; }
            for x in start_x..end_x {
                if x >= self.width { break; }
                let dx = x as f32 - center_x as f32;
                let dy = y as f32 - center_y as f32;
                let dist_sq = dx*dx + dy*dy;

                if dist_sq < r_sq {
                    let factor = 1.0 - (dist_sq / r_sq); // 1.0 at center, 0.0 at edge
                    // Reduce resistance, but don't go below 0.1
                    self.cells[y][x].resistance = (self.cells[y][x].resistance - (strength * factor)).max(0.1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_initialization() {
        let map = Map::new(10, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert!(map.cells[0][0].resistance >= 5.0);
    }

    #[test]
    fn test_add_nutrient() {
        let mut map = Map::new(10, 10);
        let initial_resistance = map.cells[5][5].resistance;
        map.add_nutrient(5, 5, 2.0, 5.0);
        assert!(map.cells[5][5].resistance < initial_resistance);
    }

    #[test]
    fn test_rhizome_step() {
        let mut map = Map::new(10, 10);
        let mut rhizome = Rhizome::new(0, 0, &mut map);

        rhizome.step(&mut map, 100);

        // Start should be visited
        assert!(map.cells[0][0].visited);
        // Neighbors should be visited or at least in frontier (but we can't check frontier easily)
        // Check if at least one neighbor is visited
        let neighbors_visited =
            map.cells[0][1].visited ||
            map.cells[1][0].visited ||
            map.cells[1][1].visited;

        assert!(neighbors_visited);
    }
}

#[derive(PartialEq, Eq)]
pub struct State {
    pub cost: NotNan<f32>,
    pub position: (usize, usize),
}

// We want min-heap, so we flip the ordering
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Rhizome {
    pub frontier: BinaryHeap<State>,
}

impl Rhizome {
    pub fn new(start_x: usize, start_y: usize, map: &mut Map) -> Self {
        let mut frontier = BinaryHeap::new();

        // Initialize start node
        map.cells[start_y][start_x].dist = 0.0;
        // Do NOT mark visited yet, let step() do it so neighbors are added.

        frontier.push(State {
            cost: NotNan::new(0.0).unwrap(),
            position: (start_x, start_y),
        });

        Self { frontier }
    }

    pub fn step(&mut self, map: &mut Map, steps: usize) {
        let mut count = 0;

        while count < steps {
            if let Some(State { cost: _, position: (x, y) }) = self.frontier.pop() {

                if map.cells[y][x].visited {
                    continue;
                }
                map.cells[y][x].visited = true;

                // If resistance is very low (nutrient), make it a taproot
                if map.cells[y][x].resistance < 1.0 {
                    self.trace_taproot(map, (x, y));
                }

                // Neighbors
                let neighbors = [
                    (x.wrapping_sub(1), y),     (x + 1, y),
                    (x, y.wrapping_sub(1)),     (x, y + 1),
                    (x.wrapping_sub(1), y.wrapping_sub(1)), (x + 1, y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y + 1), (x + 1, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx >= map.width || ny >= map.height { continue; }
                    if map.cells[ny][nx].visited { continue; }

                    let move_cost = map.cells[ny][nx].resistance * if nx != x && ny != y { 1.414 } else { 1.0 };
                    let new_dist = map.cells[y][x].dist + move_cost;

                    if new_dist < map.cells[ny][nx].dist {
                        map.cells[ny][nx].dist = new_dist;
                        map.cells[ny][nx].parent = Some((x, y));
                        self.frontier.push(State {
                            cost: NotNan::new(new_dist).unwrap(),
                            position: (nx, ny),
                        });
                    }
                }
                count += 1;
            } else {
                break;
            }
        }
    }

    pub fn trace_taproot(&self, map: &mut Map, start: (usize, usize)) {
        let (mut cx, mut cy) = start;
        let mut safety = 0;
        loop {
            // Check if already taproot to avoid cycles or re-doing work
             if map.cells[cy][cx].is_taproot {
                 break;
             }
             map.cells[cy][cx].is_taproot = true;

             if let Some((px, py)) = map.cells[cy][cx].parent {
                 cx = px;
                 cy = py;
             } else {
                 break; // Reached source
             }

            safety += 1;
            if safety > 10000 { break; }
        }
    }
}
