use chimera_lang::prelude::*;
use crate::circuit::HyperbolicCircuit;
use rand::Rng;

pub struct BioAgent {
    pub vm: ChimeraVM,
    pub current_pad: usize,
    pub target_pad: Option<usize>,
    pub progress: f64,
    pub speed: f64,
}

impl BioAgent {
    pub fn new(start_pad: usize, dna: Dna) -> Self {
        let vm = ChimeraVM::new(dna);
        // Initialize VM state if needed
        Self {
            vm,
            current_pad: start_pad,
            target_pad: None,
            progress: 0.0,
            speed: 0.02, // Travel speed along geodesic
        }
    }

    pub fn tick(&mut self, circuit: &HyperbolicCircuit) {
        if let Some(target) = self.target_pad {
            // Traveling
            self.progress += self.speed;
            if self.progress >= 1.0 {
                // Arrived
                self.current_pad = target;
                self.target_pad = None;
                self.progress = 0.0;
            }
        } else {
            // At Pad - Decision Time
            let neighbors = &circuit.adjacency[self.current_pad];
            if neighbors.is_empty() {
                // Stuck? Stay here.
                return;
            }

            // Prepare Inputs in Grid (Sensor Array)
            // Note: ChimeraVM uses a Harvard architecture (Code in DNA, Data in Grid/Stack).
            // Writing to grid[0][0] is safe and won't overwrite the program code.
            // (0,0): Number of neighbors
            self.vm.grid[0][0] = Value::Int(neighbors.len() as i64);
            // (0,1): Random noise (to break loops)
            self.vm.grid[0][1] = Value::Int(rand::thread_rng().gen_range(0..100));

            // Run VM
            self.vm.step(); // Run one step or multiple? Let's run a burst.
            for _ in 0..50 {
                if self.vm.halted { break; }
                self.vm.step();
                // Check if we have an output on stack
                if !self.vm.stack.is_empty() {
                    break;
                }
            }

            // Check Output Stack
            if let Some(val) = self.vm.stack.pop() {
                if let Value::Int(n) = val {
                    let idx = (n.abs() as usize) % neighbors.len();
                    self.target_pad = Some(neighbors[idx]);
                } else {
                    // Invalid type, pick random
                    let idx = rand::thread_rng().gen_range(0..neighbors.len());
                    self.target_pad = Some(neighbors[idx]);
                }
            } else {
                // No output, pick random
                let idx = rand::thread_rng().gen_range(0..neighbors.len());
                self.target_pad = Some(neighbors[idx]);
            }

            // Clear stack for next turn
            self.vm.stack.clear();
        }
    }
}
