
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub is_free: bool,
    pub lifetime: f32, // For visual decay (1.0 = fresh, 0.0 = old/dead)
    pub id: u32,       // Unique ID for color generation
}

pub struct Heap {
    pub blocks: Vec<Block>,
    pub total_size: usize,
    pub next_id: u32,
}

impl Heap {
    pub fn new(total_size: usize) -> Self {
        Heap {
            blocks: vec![Block {
                start: 0,
                size: total_size,
                is_free: true,
                lifetime: 0.0,
                id: 0,
            }],
            total_size,
            next_id: 1,
        }
    }

    pub fn malloc(&mut self, size: usize) -> Option<usize> {
        // First Fit Strategy
        for i in 0..self.blocks.len() {
            if self.blocks[i].is_free && self.blocks[i].size >= size {
                let start = self.blocks[i].start;
                let original_size = self.blocks[i].size;

                // Assign ID
                let id = self.next_id;
                self.next_id = self.next_id.wrapping_add(1);
                if self.next_id == 0 {
                    self.next_id = 1;
                }

                if original_size > size {
                    // Split block
                    self.blocks[i].size = size;
                    self.blocks[i].is_free = false;
                    self.blocks[i].lifetime = 1.0;
                    self.blocks[i].id = id;

                    let new_block = Block {
                        start: start + size,
                        size: original_size - size,
                        is_free: true,
                        lifetime: 0.0,
                        id: 0,
                    };
                    self.blocks.insert(i + 1, new_block);
                } else {
                    // Perfect fit
                    self.blocks[i].is_free = false;
                    self.blocks[i].lifetime = 1.0;
                    self.blocks[i].id = id;
                }

                return Some(start);
            }
        }
        None
    }

    pub fn free(&mut self, ptr: usize) {
        if let Some(pos) = self.blocks.iter().position(|b| b.start == ptr) {
            self.blocks[pos].is_free = true;
            // self.blocks[pos].lifetime = 1.0; // Reset lifetime for "decay" animation?
            // Actually, maybe we want it to flash when freed.
            self.blocks[pos].lifetime = 1.0;
            self.coalesce();
        }
    }

    fn coalesce(&mut self) {
        let mut i = 0;
        while i < self.blocks.len() - 1 {
            if self.blocks[i].is_free && self.blocks[i + 1].is_free {
                let next_size = self.blocks[i + 1].size;
                self.blocks[i].size += next_size;
                self.blocks.remove(i + 1);
                // Don't increment i, check current against new next
            } else {
                i += 1;
            }
        }
    }

    pub fn update_lifetimes(&mut self, delta: f32) {
        for block in &mut self.blocks {
            if block.lifetime > 0.0 {
                block.lifetime -= delta;
                if block.lifetime < 0.0 {
                    block.lifetime = 0.0;
                }
            }
        }
    }
}

impl fmt::Display for Heap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Heap {{ size: {}, blocks: {} }}", self.total_size, self.blocks.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malloc_simple() {
        let mut heap = Heap::new(100);
        let ptr = heap.malloc(10).unwrap();
        assert_eq!(ptr, 0);
        assert_eq!(heap.blocks.len(), 2); // One allocated, one remaining free
        assert_eq!(heap.blocks[0].size, 10);
        assert_eq!(heap.blocks[0].is_free, false);
        assert_eq!(heap.blocks[1].size, 90);
        assert_eq!(heap.blocks[1].is_free, true);
    }

    #[test]
    fn test_malloc_full() {
        let mut heap = Heap::new(10);
        let ptr = heap.malloc(10).unwrap();
        assert_eq!(ptr, 0);
        assert_eq!(heap.blocks.len(), 1);
        assert!(heap.malloc(1).is_none());
    }

    #[test]
    fn test_free_coalesce() {
        let mut heap = Heap::new(100);
        let p1 = heap.malloc(10).unwrap();
        let p2 = heap.malloc(20).unwrap();
        let _p3 = heap.malloc(30).unwrap();

        // Free middle
        heap.free(p2);
        assert_eq!(heap.blocks.len(), 4); // p1, free(p2), p3, rest

        // Free first
        heap.free(p1);
        // Should merge p1 and p2
        assert_eq!(heap.blocks.len(), 3); // free(p1+p2), p3, rest
        assert_eq!(heap.blocks[0].size, 30);
        assert_eq!(heap.blocks[0].is_free, true);
    }
}
