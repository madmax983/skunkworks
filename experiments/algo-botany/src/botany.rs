use crate::grid::{Grid, CellType};
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, PartialEq)]
pub struct State {
    pub cost: f32,
    pub position: (usize, usize),
}

impl Eq for State {}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        if other.cost < self.cost {
            Ordering::Less
        } else if other.cost > self.cost {
            Ordering::Greater
        } else {
            // Position comparison just to be deterministic
            self.position.cmp(&other.position)
        }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Plant {
    pub open_set: BinaryHeap<State>,
    pub came_from: HashMap<(usize, usize), (usize, usize)>,
    pub g_score: HashMap<(usize, usize), f32>,
    pub path: Vec<(usize, usize)>,
    pub finished: bool,
    pub growth_speed: usize, // Nodes to process per frame
}

impl Plant {
    pub fn new(start: (usize, usize)) -> Self {
        let mut open_set = BinaryHeap::new();
        open_set.push(State {
            cost: 0.0,
            position: start,
        });

        let mut g_score = HashMap::new();
        g_score.insert(start, 0.0);

        Self {
            open_set,
            came_from: HashMap::new(),
            g_score,
            path: Vec::new(),
            finished: false,
            growth_speed: 1,
        }
    }

    fn heuristic(&self, a: (usize, usize), b: (usize, usize)) -> f32 {
        // Manhattan distance
        ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as f32
    }

    pub fn update(&mut self, grid: &Grid) {
        if self.finished || self.open_set.is_empty() {
            return;
        }

        // Process a few nodes per frame for animation
        for _ in 0..self.growth_speed {
            if let Some(State { cost: _f, position: current }) = self.open_set.pop() {

                // Check if goal reached
                if let Some(goal) = grid.goal {
                    if current == goal {
                        self.reconstruct_path(current);
                        self.finished = true;
                        return;
                    }
                }

                // Expand neighbors
                for neighbor in grid.neighbors(current.0, current.1) {
                    let neighbor_cell = grid.get(neighbor.0, neighbor.1).unwrap();
                    if neighbor_cell.cell_type == CellType::Rock {
                        continue;
                    }

                    let tentative_g = self.g_score[&current] + neighbor_cell.cost();

                    if tentative_g < *self.g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                        self.came_from.insert(neighbor, current);
                        self.g_score.insert(neighbor, tentative_g);

                        let h = if let Some(goal) = grid.goal {
                            self.heuristic(neighbor, goal)
                        } else {
                            0.0
                        };

                        self.open_set.push(State {
                            cost: tentative_g + h,
                            position: neighbor,
                        });
                    }
                }
            } else {
                // Open set empty, no path found
                self.finished = true;
                return;
            }
        }
    }

    fn reconstruct_path(&mut self, mut current: (usize, usize)) {
        self.path.push(current);
        while let Some(&parent) = self.came_from.get(&current) {
            current = parent;
            self.path.push(current);
        }
        self.path.reverse();
    }

    pub fn draw(&self, cell_size: f32, offset_x: f32, offset_y: f32) {
        // Draw roots (came_from)
        for (&child, &parent) in &self.came_from {
            draw_line(
                parent.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                parent.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                child.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                child.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                2.0,
                Color::new(0.6, 0.4, 0.2, 0.8), // Root color
            );
        }

        // Draw active tips (open_set)
        // Note: Iterating binary heap is not ordered, but fine for drawing
        for state in &self.open_set {
            draw_circle(
                state.position.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                state.position.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                cell_size / 4.0,
                Color::new(0.2, 0.8, 0.2, 0.8), // Bud color
            );
        }

        // Draw final path if finished
        if self.finished && !self.path.is_empty() {
            for i in 0..self.path.len() - 1 {
                let p1 = self.path[i];
                let p2 = self.path[i+1];
                draw_line(
                    p1.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                    p1.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                    p2.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                    p2.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                    4.0,
                    GOLD, // Main vein
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plant_initialization() {
        let plant = Plant::new((0, 0));
        assert_eq!(plant.open_set.len(), 1);
        assert_eq!(plant.g_score.len(), 1);
    }

    #[test]
    fn test_plant_growth() {
        let mut grid = Grid::new(5, 5);
        grid.goal = Some((2, 0));
        let mut plant = Plant::new((0, 0));

        // Step 1
        plant.update(&grid);
        // Neighbors of (0,0) are (0,1) and (1,0) added to open set
        assert!(plant.open_set.len() > 0);
        assert!(plant.came_from.len() > 0);
    }

    #[test]
    fn test_path_finding() {
        let mut grid = Grid::new(5, 5);
        grid.goal = Some((4, 0));
        let mut plant = Plant::new((0, 0));

        // Let it grow enough times
        for _ in 0..100 {
            plant.update(&grid);
            if plant.finished {
                break;
            }
        }

        assert!(plant.finished);
        assert!(plant.path.len() > 0);
        // Path should be (0,0) -> (1,0) -> (2,0) -> (3,0) -> (4,0) (length 5)
        // Or similar length
        assert_eq!(plant.path.last(), Some(&(4,0)));
    }
}
