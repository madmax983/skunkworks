use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub is_allocated: bool,
}

#[derive(Debug)]
pub struct Heap {
    pub blocks: Vec<Block>,
    pub total_size: usize,
}

impl Heap {
    pub fn new(total_size: usize) -> Self {
        Self {
            blocks: vec![Block {
                start: 0,
                size: total_size,
                is_allocated: false,
            }],
            total_size,
        }
    }

    /// Tries to allocate a block of `size`.
    /// Returns Some(index) if successful, None if no space.
    /// Uses First-Fit strategy.
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        for i in 0..self.blocks.len() {
            if !self.blocks[i].is_allocated && self.blocks[i].size >= size {
                // Found a block!
                let original_size = self.blocks[i].size;
                let original_start = self.blocks[i].start;

                if original_size == size {
                    // Exact match
                    self.blocks[i].is_allocated = true;
                    return Some(i);
                } else {
                    // Split
                    self.blocks[i].size = size;
                    self.blocks[i].is_allocated = true;

                    let new_block = Block {
                        start: original_start + size,
                        size: original_size - size,
                        is_allocated: false,
                    };
                    self.blocks.insert(i + 1, new_block);
                    return Some(i);
                }
            }
        }
        None
    }

    /// Frees the block at the given index.
    pub fn free(&mut self, index: usize) {
        if index < self.blocks.len() {
            self.blocks[index].is_allocated = false;
        }
    }

    /// Randomly frees an allocated block.
    pub fn random_free(&mut self) {
        let allocated_indices: Vec<usize> = self
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if b.is_allocated { Some(i) } else { None })
            .collect();

        if allocated_indices.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let idx = allocated_indices[rng.gen_range(0..allocated_indices.len())];
        self.free(idx);
    }

    /// Returns the fragmentation percentage (0.0 to 1.0).
    /// Calculated as: 1.0 - (largest_free_block / total_free_memory)
    pub fn fragmentation(&self) -> f64 {
        let mut max_free = 0;
        let mut total_free = 0;

        for block in &self.blocks {
            if !block.is_allocated {
                total_free += block.size;
                if block.size > max_free {
                    max_free = block.size;
                }
            }
        }

        if total_free == 0 {
            return 0.0;
        }

        1.0 - (max_free as f64 / total_free as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_exact() {
        let mut heap = Heap::new(100);
        let idx = heap.allocate(100).unwrap();
        assert_eq!(heap.blocks[idx].is_allocated, true);
        assert_eq!(heap.blocks.len(), 1);
    }

    #[test]
    fn test_allocate_split() {
        let mut heap = Heap::new(100);
        let idx = heap.allocate(40).unwrap();

        assert_eq!(idx, 0);
        assert_eq!(heap.blocks[0].size, 40);
        assert_eq!(heap.blocks[0].is_allocated, true);

        assert_eq!(heap.blocks[1].size, 60);
        assert_eq!(heap.blocks[1].is_allocated, false);
    }

    #[test]
    fn test_allocate_fail() {
        let mut heap = Heap::new(10);
        let idx = heap.allocate(20);
        assert!(idx.is_none());
    }

    #[test]
    fn test_free() {
        let mut heap = Heap::new(100);
        let idx = heap.allocate(50).unwrap();
        heap.free(idx);
        assert_eq!(heap.blocks[0].is_allocated, false);
    }
}
