#![allow(clippy::collapsible_if)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::model::{Grid, SoilType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Dijkstra,
    AStar,
    Greedy,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize, // using integer cost for stability and Ord
    pub position: usize, // index in grid
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct RootSystem {
    pub algorithm: Algorithm,
    pub frontier: BinaryHeap<State>,
    pub visited: HashMap<usize, usize>, // child_idx -> parent_idx
    pub start_idx: usize,
    pub target_idx: Option<usize>, // For A*
    pub finished: bool,
}

impl RootSystem {
    pub fn new(algorithm: Algorithm, start_x: usize, start_y: usize, grid_width: usize) -> Self {
        let start_idx = start_y * grid_width + start_x;
        let mut frontier = BinaryHeap::new();

        frontier.push(State { cost: 0, position: start_idx });

        let mut visited = HashMap::new();
        visited.insert(start_idx, start_idx); // root's parent is itself

        Self {
            algorithm,
            frontier,
            visited,
            start_idx,
            target_idx: None, // Will be set later or unused
            finished: false,
        }
    }

    pub fn set_target(&mut self, x: usize, y: usize, grid_width: usize) {
        self.target_idx = Some(y * grid_width + x);
    }

    pub fn step(&mut self, grid: &mut Grid, root_id: usize) {
        if self.frontier.is_empty() {
            self.finished = true;
            return;
        }

        // Process one node
        if let Some(State { cost, position }) = self.frontier.pop() {
            let cx = position % grid.width;
            let cy = position / grid.width;

            // Check if we reached target (optional stopping condition)
            if let Some(target) = self.target_idx {
                if position == target {
                    self.finished = true; // Don't stop completely, maybe continue to fill?
                    // For now, let's keep growing to visualize the whole tree or until resource exhausted
                }
            }

            // Neighbors (Up, Down, Left, Right)
            let neighbors = [
                (cx.wrapping_sub(1), cy),
                (cx + 1, cy),
                (cx, cy.wrapping_sub(1)),
                (cx, cy + 1),
            ];

            for (nx, ny) in neighbors {
                if nx >= grid.width || ny >= grid.height {
                    continue;
                }

                let n_idx = ny * grid.width + nx;

                if self.visited.contains_key(&n_idx) {
                    continue;
                }

                // Get cell cost
                let cell = &mut grid.cells[n_idx];
                if cell.kind == SoilType::Rock {
                    continue; // Cannot grow through bedrock
                }

                if let Some(occupier) = cell.occupied_by {
                    if occupier != root_id {
                         // Collision with another root!
                         // For now, treat it like rock (can't cross)
                         continue;
                    }
                }

                // Mark as occupied by us?
                // Wait, if we mark it now, we might block ourselves?
                // No, we check visited. But wait, `step` is called iteratively.
                // If we add to frontier, we haven't "grown" there yet physically?
                // Visualizing: Frontier are "potential" growth. Visited is "actual" root structure.
                // So we should only mark as occupied when we POP from frontier?
                // The current logic: pop `position`, then explore neighbors.
                // So `position` is definitely part of the root now.
                // Let's mark `position` as occupied.

                // However, `position` was popped from frontier. It was added previously.
                // We should mark `n_idx` as occupied when we add it to visited.

                // Let's refine:
                // When we `push` to frontier, it's a "tendril".
                // When we `pop`, it becomes "wood".

                // Actually, simplest is: if in `visited`, it is occupied.
                // But `visited` is local. So we need to write to Grid.

                // Let's mark `n_idx` as occupied immediately when adding to visited.
                // This reserves the spot.

                // Cost calculation
                // Base cost is 1. Density adds penalty.
                // Density 0.0 -> Cost 10
                // Density 1.0 -> Cost 100
                let move_cost = 10 + (cell.density * 90.0) as usize;
                let new_cost = cost + move_cost;

                // Heuristic for A* / Greedy
                let heuristic = if self.algorithm == Algorithm::Dijkstra {
                    0
                } else if let Some(t_idx) = self.target_idx {
                    let tx = t_idx % grid.width;
                    let ty = t_idx / grid.width;
                    let dist = ((nx as isize - tx as isize).abs() + (ny as isize - ty as isize).abs()) as usize;

                    if self.algorithm == Algorithm::Greedy {
                        dist * 20 // Heavily weight distance
                    } else {
                        dist * 10 // A*: f = g + h
                    }
                } else {
                    0
                };

                let priority = match self.algorithm {
                    Algorithm::Dijkstra => new_cost,
                    Algorithm::AStar => new_cost + heuristic,
                    Algorithm::Greedy => heuristic + move_cost, // Local greedy
                };

                self.visited.insert(n_idx, position);
                grid.cells[n_idx].occupied_by = Some(root_id); // Mark grid
                self.frontier.push(State { cost: priority, position: n_idx });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Grid, SoilCell, SoilType};

    #[test]
    fn test_root_avoidance() {
        // Create a small 5x5 grid
        let mut grid = Grid::new(5, 5);
        // Place a rock wall at x=2, leaving a gap at y=0
        for y in 1..5 {
            let idx = y * 5 + 2;
            grid.cells[idx] = SoilCell { density: 1.0, moisture: 0.0, kind: SoilType::Rock, occupied_by: None };
        }

        let mut root = RootSystem::new(Algorithm::Dijkstra, 0, 2, 5);
        root.set_target(4, 2, 5);

        // Step enough times to go around
        for _ in 0..100 {
            root.step(&mut grid, 0);
        }

        // Check if we reached the target
        let target_idx = 2 * 5 + 4;
        assert!(root.visited.contains_key(&target_idx), "Root should have reached the target");

        // Check if we avoided the wall
        let wall_center = 2 * 5 + 2;
        assert!(!root.visited.contains_key(&wall_center), "Root should not grow into rock");

        // Check path finding logic (simplified)
        // Ensure it went through the gap at (2,0)
        let gap_idx = 0 * 5 + 2;
        assert!(root.visited.contains_key(&gap_idx), "Root should have passed through the gap");
    }
}
