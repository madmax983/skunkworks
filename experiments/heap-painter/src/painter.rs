use crate::allocator::{Heap, AllocationId};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushMode {
    Chaos,
    Linear,
    Wave,
}

impl std::fmt::Display for BrushMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrushMode::Chaos => write!(f, "Chaos"),
            BrushMode::Linear => write!(f, "Linear"),
            BrushMode::Wave => write!(f, "Wave"),
        }
    }
}

pub struct Painter {
    pub heap: Heap,
    pub mode: BrushMode,
    pub allocated_ids: Vec<AllocationId>,
    pub frame: usize,
}

impl Painter {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            heap: Heap::new(width, height),
            mode: BrushMode::Chaos,
            allocated_ids: Vec::new(),
            frame: 0,
        }
    }

    pub fn tick(&mut self) {
        self.heap.tick();
        self.frame += 1;

        match self.mode {
            BrushMode::Chaos => self.chaos_tick(),
            BrushMode::Linear => self.linear_tick(),
            BrushMode::Wave => self.wave_tick(),
        }
    }

    fn chaos_tick(&mut self) {
        let mut rng = rand::thread_rng();

        // Randomly free some
        if !self.allocated_ids.is_empty() && rng.gen_bool(0.3) {
            let idx = rng.gen_range(0..self.allocated_ids.len());
            let id = self.allocated_ids.remove(idx);
            self.heap.free(id);
        }

        // Randomly allocate
        if rng.gen_bool(0.7) {
            let size = rng.gen_range(1..50);
            if let Some(id) = self.heap.malloc(size) {
                self.allocated_ids.push(id);
            }
        }
    }

    fn linear_tick(&mut self) {
         let mut rng = rand::thread_rng();
         // Steady allocation
         let size = rng.gen_range(5..20);
         if let Some(id) = self.heap.malloc(size) {
             self.allocated_ids.push(id);
         }

         // Steady free (FIFO-ish)
         if self.allocated_ids.len() > 100 {
             let id = self.allocated_ids.remove(0);
             self.heap.free(id);
         }
    }

    fn wave_tick(&mut self) {
        // Allocates based on sine wave of sizes
        let size = (((self.frame as f64 * 0.1).sin() + 1.0) * 10.0) as usize + 1;
        if let Some(id) = self.heap.malloc(size) {
            self.allocated_ids.push(id);
        }

        // Free random older ones to maintain a certain load
         if self.allocated_ids.len() > 150 {
             let id = self.allocated_ids.remove(0);
             self.heap.free(id);
         }
    }

    pub fn switch_mode(&mut self) {
        self.mode = match self.mode {
            BrushMode::Chaos => BrushMode::Linear,
            BrushMode::Linear => BrushMode::Wave,
            BrushMode::Wave => BrushMode::Chaos,
        };
        // Optionally clear heap or keep it mixed? Keeping it mixed is more artistic.
    }
}
