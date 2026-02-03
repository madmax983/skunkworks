use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllocationId(pub usize);

#[derive(Debug, Clone)]
pub struct Allocation {
    pub id: AllocationId,
    pub start: usize,
    pub size: usize,
    pub age: usize,
    pub color_seed: u8,
}

pub struct Heap {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Option<AllocationId>>,
    pub allocations: HashMap<AllocationId, Allocation>,
    next_id: usize,
}

impl Heap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![None; width * height],
            allocations: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn malloc(&mut self, size: usize) -> Option<AllocationId> {
        let total_size = self.width * self.height;
        if size == 0 || size > total_size {
            return None;
        }

        let mut run_start = 0;
        let mut run_length = 0;

        for i in 0..total_size {
            if self.grid[i].is_none() {
                if run_length == 0 {
                    run_start = i;
                }
                run_length += 1;
                if run_length == size {
                    // Found a suitable block
                    let id = AllocationId(self.next_id);
                    self.next_id += 1;

                    let mut rng = rand::thread_rng();
                    let allocation = Allocation {
                        id,
                        start: run_start,
                        size,
                        age: 0,
                        color_seed: rng.gen(),
                    };

                    self.allocations.insert(id, allocation);
                    for j in run_start..(run_start + size) {
                        self.grid[j] = Some(id);
                    }
                    return Some(id);
                }
            } else {
                run_length = 0;
            }
        }
        None
    }

    pub fn free(&mut self, id: AllocationId) {
        if let Some(alloc) = self.allocations.remove(&id) {
            for i in alloc.start..(alloc.start + alloc.size) {
                self.grid[i] = None;
            }
        }
    }

    pub fn tick(&mut self) {
        for alloc in self.allocations.values_mut() {
            alloc.age = alloc.age.saturating_add(1);
        }
    }

    pub fn fragmentation(&self) -> f64 {
        // A simple metric: 1.0 - (largest_free_block / total_free_memory)
        let total_size = self.width * self.height;
        let mut free_cells = 0;
        let mut max_run = 0;
        let mut current_run = 0;

        for i in 0..total_size {
            if self.grid[i].is_none() {
                free_cells += 1;
                current_run += 1;
            } else {
                max_run = max_run.max(current_run);
                current_run = 0;
            }
        }
        max_run = max_run.max(current_run);

        if free_cells == 0 {
            return 0.0;
        }

        1.0 - (max_run as f64 / free_cells as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malloc_success() {
        let mut heap = Heap::new(10, 1);
        let id1 = heap.malloc(5).expect("Should allocate 5");
        assert_eq!(heap.allocations.len(), 1);
        assert_eq!(heap.grid[0], Some(id1));
        assert_eq!(heap.grid[4], Some(id1));
        assert_eq!(heap.grid[5], None);
    }

    #[test]
    fn test_malloc_oom() {
        let mut heap = Heap::new(10, 1);
        let id1 = heap.malloc(11);
        assert!(id1.is_none());
    }

    #[test]
    fn test_free() {
        let mut heap = Heap::new(10, 1);
        let id1 = heap.malloc(5).unwrap();
        heap.free(id1);
        assert_eq!(heap.allocations.len(), 0);
        assert!(heap.grid.iter().all(|x| x.is_none()));
    }

    #[test]
    fn test_fragmentation() {
        let mut heap = Heap::new(10, 1);
        // [X X X X X _ _ _ _ _]
        let id1 = heap.malloc(5).unwrap();
        // [X X X X X X X _ _ _]
        let id2 = heap.malloc(2).unwrap();
        // [X X X X X X X X X _]
        let id3 = heap.malloc(2).unwrap();

        // Free middle
        // [X X X X X _ _ X X _]
        heap.free(id2);

        // Free cells: 3 (indices 5, 6, 9)
        // Max run: 2 (indices 5, 6)
        // Frag: 1.0 - (2/3) = 1 - 0.66 = 0.33
        let frag = heap.fragmentation();
        assert!((frag - 0.333).abs() < 0.01);
    }

    #[test]
    fn test_tick() {
        let mut heap = Heap::new(10, 1);
        let id = heap.malloc(5).unwrap();
        heap.tick();
        assert_eq!(heap.allocations[&id].age, 1);
    }
}
