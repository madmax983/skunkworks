use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub free: bool,
    pub color: Color,
    pub age: f32, // For visualization effects
}

pub struct Heap {
    pub capacity: usize,
    pub blocks: Vec<Block>,
}

impl Heap {
    pub fn new(capacity: usize) -> Self {
        Heap {
            capacity,
            blocks: vec![Block {
                start: 0,
                size: capacity,
                free: true,
                color: BLACK, // Background color
                age: 0.0,
            }],
        }
    }

    pub fn malloc(&mut self, size: usize, color: Color) -> Option<usize> {
        let mut best_fit_idx = None;

        // Find first fit
        for (i, block) in self.blocks.iter().enumerate() {
            if block.free && block.size >= size {
                best_fit_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = best_fit_idx {
            let current_start = self.blocks[idx].start;

            if self.blocks[idx].size > size {
                // Split
                let current_size = self.blocks[idx].size;

                // Modify current block to be the allocated part
                self.blocks[idx].size = size;
                self.blocks[idx].free = false;
                self.blocks[idx].color = color;
                self.blocks[idx].age = 0.0;

                // Insert new free block after
                let new_block = Block {
                    start: current_start + size,
                    size: current_size - size,
                    free: true,
                    color: BLACK,
                    age: 0.0,
                };
                self.blocks.insert(idx + 1, new_block);
            } else {
                // Exact fit
                self.blocks[idx].free = false;
                self.blocks[idx].color = color;
                self.blocks[idx].age = 0.0;
            }
            return Some(current_start);
        }

        None
    }

    pub fn free(&mut self, addr: usize) {
        let mut block_idx = None;
        for (i, block) in self.blocks.iter().enumerate() {
            if block.start == addr {
                block_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = block_idx {
            if !self.blocks[idx].free {
                self.blocks[idx].free = true;
                self.blocks[idx].color = BLACK;
                self.coalesce();
            }
        }
    }

    fn coalesce(&mut self) {
        let mut i = 0;
        while i < self.blocks.len() - 1 {
            // Need to check bounds carefully as len changes
            if self.blocks[i].free && self.blocks[i + 1].free {
                self.blocks[i].size += self.blocks[i + 1].size;
                self.blocks.remove(i + 1);
                // Don't increment i, check current i against new neighbor
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_heap() {
        let heap = Heap::new(100);
        assert_eq!(heap.blocks.len(), 1);
        assert_eq!(heap.blocks[0].size, 100);
        assert!(heap.blocks[0].free);
    }

    #[test]
    fn test_malloc_split() {
        let mut heap = Heap::new(100);
        let addr = heap.malloc(10, BLACK).unwrap();

        assert_eq!(addr, 0);
        assert_eq!(heap.blocks.len(), 2);

        // Allocated block
        assert_eq!(heap.blocks[0].start, 0);
        assert_eq!(heap.blocks[0].size, 10);
        assert!(!heap.blocks[0].free);

        // Free block
        assert_eq!(heap.blocks[1].start, 10);
        assert_eq!(heap.blocks[1].size, 90);
        assert!(heap.blocks[1].free);
    }

    #[test]
    fn test_malloc_full() {
        let mut heap = Heap::new(10);
        let addr = heap.malloc(10, BLACK).unwrap();
        assert_eq!(addr, 0);
        assert_eq!(heap.blocks.len(), 1);
        assert!(!heap.blocks[0].free);

        let addr2 = heap.malloc(1, BLACK);
        assert!(addr2.is_none());
    }

    #[test]
    fn test_free_and_coalesce() {
        let mut heap = Heap::new(100);
        let addr1 = heap.malloc(10, BLACK).unwrap();
        let addr2 = heap.malloc(20, BLACK).unwrap();
        let _addr3 = heap.malloc(30, BLACK).unwrap();

        assert_eq!(heap.blocks.len(), 4); // [Alloc(10), Alloc(20), Alloc(30), Free(40)]

        heap.free(addr1);
        assert_eq!(heap.blocks[0].free, true);
        assert_eq!(heap.blocks.len(), 4); // No coalesce yet

        heap.free(addr2);
        // addr1 and addr2 are now free and adjacent. Should coalesce.
        // blocks: [Free(10), Free(20), Alloc(30), Free(40)] -> [Free(30), Alloc(30), Free(40)]

        assert_eq!(heap.blocks.len(), 3);
        assert_eq!(heap.blocks[0].size, 30);
        assert!(heap.blocks[0].free);
        assert!(!heap.blocks[1].free);
    }
}
