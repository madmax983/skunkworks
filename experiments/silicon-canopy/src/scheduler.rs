use macroquad::prelude::Color;
use crate::grid::{Grid, Cell};
use crate::root::RootAgent;

pub struct Process {
    pub id: usize,
    pub color: Color,
    pub root_agent: RootAgent,
    pub resources: usize,
    pub io_ops: usize,
}

pub struct Scheduler {
    pub processes: Vec<Process>,
    pub current_idx: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_idx: 0,
        }
    }

    pub fn spawn_process(&mut self, grid: &mut Grid, id: usize, x: usize, y: usize, color: Color) {
        let agent = RootAgent::new(x, y);
        // Mark grid starting position
        grid.set(x, y, Cell::Root(id));

        self.processes.push(Process {
            id,
            color,
            root_agent: agent,
            resources: 0,
            io_ops: 0,
        });
    }

    pub fn tick(&mut self, grid: &mut Grid) {
        if self.processes.is_empty() {
            return;
        }

        let process = &mut self.processes[self.current_idx];

        // Try to grow
        if let Some(cell) = process.root_agent.grow_step(grid, process.id) {
            match cell {
                Cell::Memory => process.resources += 1,
                Cell::IO => process.io_ops += 1,
                _ => {}
            }
        }

        // Round Robin
        self.current_idx = (self.current_idx + 1) % self.processes.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{Grid, Cell};
    use macroquad::prelude::RED;

    #[test]
    fn test_scheduling() {
        let mut scheduler = Scheduler::new();
        let mut grid = Grid::new(10, 10);

        // Clear grid
        for c in grid.cells.iter_mut() { *c = Cell::Empty; }

        // Place resource nearby
        grid.set(1, 0, Cell::Memory);

        scheduler.spawn_process(&mut grid, 1, 0, 0, RED);

        // Tick 1: Process 1 acts (idx 0)
        scheduler.tick(&mut grid);

        // Should have found target, but not moved yet (find_target runs, path populated)
        // Actually, grow_step runs find_target internally if path empty.
        // If path found, it *also* takes the first step?
        // Let's check root.rs:
        // if self.path.is_empty() { find_target... }
        // if let Some(next_pos) = self.path.last() { move... }
        // Yes, it moves immediately in the same tick if path found.

        // Path from (0,0) to (1,0) is likely direct.
        // Step 1: moves to (1,0). Consumes Memory.

        assert_eq!(scheduler.processes[0].resources, 1);
        assert_eq!(scheduler.processes[0].root_agent.position, (1, 0));

        // Scheduler should rotate
        assert_eq!(scheduler.current_idx, 0); // (0 + 1) % 1 = 0
    }
}
