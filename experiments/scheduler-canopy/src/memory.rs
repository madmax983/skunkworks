use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

pub struct MemoryGrid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Option<usize>>, // None = Free, Some(pid) = Occupied
}

impl MemoryGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![None; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.grid[y * self.width + x]
    }

    pub fn is_free(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.grid[y * self.width + x].is_none()
    }

    pub fn set(&mut self, x: usize, y: usize, pid: usize) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = Some(pid);
        }
    }

    pub fn deallocate(&mut self, pid: usize) {
        for cell in self.grid.iter_mut() {
            if *cell == Some(pid) {
                *cell = None;
            }
        }
    }

    /// Attempts to allocate `amount` new cells starting growth from `tips`.
    /// Returns the newly allocated positions.
    pub fn allocate_from(&mut self, tips: &Vec<Pos>, amount: usize, pid: usize) -> Vec<Pos> {
        let mut allocated = Vec::new();
        let mut rng = thread_rng();

        if tips.is_empty() {
             return allocated;
        }

        // Use a local copy of tips to allow immediate branching within this call
        let mut active_tips = tips.clone();

        let mut attempts = 0;
        let max_attempts = amount * 50; // Give up eventually

        while allocated.len() < amount && attempts < max_attempts {
            attempts += 1;

            // Pick a random tip
            if active_tips.is_empty() { break; }
            let tip_idx = rng.gen_range(0..active_tips.len());
            let tip = active_tips[tip_idx];

            // Pick a random neighbor
            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            let (dx, dy) = dirs[rng.gen_range(0..4)];

            let nx = tip.x as isize + dx;
            let ny = tip.y as isize + dy;

            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if self.is_free(nx, ny) {
                    self.set(nx, ny, pid);
                    let new_pos = Pos { x: nx, y: ny };
                    allocated.push(new_pos);
                    active_tips.push(new_pos); // Allow growing from new growth immediately
                }
            }
        }

        allocated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation() {
        let mut grid = MemoryGrid::new(10, 10);
        let start = Pos { x: 5, y: 5 };
        grid.set(5, 5, 1); // Plant seed
        let tips = vec![start];

        let new_allocs = grid.allocate_from(&tips, 5, 1);
        assert!(new_allocs.len() <= 5);
        if !new_allocs.is_empty() {
            for p in &new_allocs {
                assert_eq!(grid.get(p.x, p.y), Some(1));
            }
        }
    }

    #[test]
    fn test_collision() {
        let mut grid = MemoryGrid::new(3, 3);
        // Fill grid with pid 2 except center
        for y in 0..3 {
            for x in 0..3 {
                if x != 1 || y != 1 {
                    grid.set(x, y, 2);
                }
            }
        }

        let start = Pos { x: 1, y: 1 };
        grid.set(1, 1, 1);
        let tips = vec![start];

        // Try to grow pid 1
        let new_allocs = grid.allocate_from(&tips, 1, 1);
        assert_eq!(new_allocs.len(), 0); // Should be trapped
    }
}
