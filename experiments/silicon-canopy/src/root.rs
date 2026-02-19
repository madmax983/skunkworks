use crate::grid::{Grid, Cell};
use std::collections::{VecDeque, HashSet, HashMap};

#[derive(Debug, Clone)]
pub struct RootAgent {
    pub position: (usize, usize),
    pub path: Vec<(usize, usize)>, // Stores path in reverse order: [target, ..., next_step]
    pub target: Option<(usize, usize)>,
}

impl RootAgent {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position: (x, y),
            path: Vec::new(),
            target: None,
        }
    }

    // Finds the nearest resource (Memory or IO)
    pub fn find_target(&mut self, grid: &Grid) {
        if self.target.is_some() && !self.path.is_empty() {
            return; // Already has target and path
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(self.position);
        visited.insert(self.position);

        while let Some(current) = queue.pop_front() {
            if let Some(cell) = grid.get(current.0, current.1) {
                if matches!(cell, Cell::Memory | Cell::IO) {
                    // Found it! Backtrace.
                    self.target = Some(current);

                    let mut path = Vec::new();
                    let mut backtrack = current;
                    while backtrack != self.position {
                        path.push(backtrack);
                        if let Some(&prev) = parent.get(&backtrack) {
                            backtrack = prev;
                        } else {
                            break; // Should not happen
                        }
                    }
                    // Path is now [target, ..., next_step]
                    self.path = path;
                    return;
                }
            }

            for neighbor in grid.neighbors(current.0, current.1) {
                if !visited.contains(&neighbor) {
                    // Check if obstacle
                    if let Some(cell) = grid.get(neighbor.0, neighbor.1) {
                        match cell {
                            Cell::Root(_) | Cell::Corrupt => continue, // Obstacle
                            _ => {
                                visited.insert(neighbor);
                                parent.insert(neighbor, current);
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn grow_step(&mut self, grid: &mut Grid, process_id: usize) -> Option<Cell> {
        if self.path.is_empty() {
            // Need to re-plan?
            self.target = None;
            self.find_target(grid);
            if self.path.is_empty() {
                return None; // No path found or no resources
            }
        }

        if let Some(next_pos) = self.path.last() {
            let (nx, ny) = *next_pos;
            // Check if still valid (race condition: another root took it?)
            if let Some(cell) = grid.get(nx, ny) {
                match cell {
                    Cell::Empty | Cell::Memory | Cell::IO => {
                        // Move
                        grid.set(nx, ny, Cell::Root(process_id));
                        self.position = (nx, ny);
                        self.path.pop(); // Remove the step we just took

                        // If we reached target, clear target
                        if self.path.is_empty() {
                            self.target = None;
                        }
                        return Some(cell);
                    }
                    _ => {
                        // Blocked, invalidate path
                        self.path.clear();
                        self.target = None;
                        return None;
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{Grid, Cell};

    #[test]
    fn test_pathfinding() {
        let mut grid = Grid::new(5, 5);
        // Clear grid to be deterministic
        for c in grid.cells.iter_mut() { *c = Cell::Empty; }

        // Set a target
        grid.set(4, 4, Cell::Memory);

        let mut agent = RootAgent::new(0, 0);
        agent.find_target(&grid);

        assert!(agent.target.is_some());
        assert_eq!(agent.target, Some((4, 4)));
        assert!(!agent.path.is_empty());

        // Grow one step
        let consumed = agent.grow_step(&mut grid, 1);
        assert!(consumed.is_some());
        assert_ne!(agent.position, (0, 0));
    }
}
