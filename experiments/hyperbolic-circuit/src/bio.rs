use chimera_lang::prelude::*;
use num_complex::Complex;
use poincare_disk::{hyperbolic_dist, mobius_add, mobius_sub, Point};
use crate::circuit::HyperCircuit;

pub struct BioAgent {
    pub vm: ChimeraVM,
    pub pos: Point,
    pub heading: f64, // Angle in local tangent space
    pub target_idx: Option<usize>,
    pub id: u64,
}

impl BioAgent {
    pub fn new(pos: Point, dna: Dna) -> Self {
        Self {
            vm: ChimeraVM::new(dna),
            pos,
            heading: rand::random::<f64>() * std::f64::consts::PI * 2.0,
            target_idx: None,
            id: rand::random(),
        }
    }

    pub fn update(&mut self, circuit: &HyperCircuit) {
        if self.vm.halted { return; }

        // --- SENSORS (Input to Grid) ---

        // 1. Find target
        if self.target_idx.is_none() {
             // Pick random pad that isn't too close (not current pad)
             let mut rng = rand::thread_rng();
             use rand::Rng;
             if !circuit.pads.is_empty() {
                 let idx = rng.gen_range(0..circuit.pads.len());
                 if hyperbolic_dist(self.pos, circuit.pads[idx]) > 0.1 {
                    self.target_idx = Some(idx);
                 }
             }
        }

        let target_pos = if let Some(idx) = self.target_idx {
            circuit.pads[idx]
        } else {
            Point::new(0.0, 0.0)
        };

        let dist = hyperbolic_dist(self.pos, target_pos);

        // Angle to target
        // We map target to the agent's local frame (where agent is at origin)
        // mobius_sub(z, a) maps a -> 0.
        // We want target (z) relative to self (a).
        let local_target_vec = mobius_sub(target_pos, self.pos);
        let angle_to_target = local_target_vec.arg();
        let mut angle_diff = angle_to_target - self.heading;

        // Normalize angle diff to -PI..PI
        while angle_diff > std::f64::consts::PI { angle_diff -= 2.0 * std::f64::consts::PI; }
        while angle_diff < -std::f64::consts::PI { angle_diff += 2.0 * std::f64::consts::PI; }

        self.vm.grid[0][0] = Value::Int((dist * 100.0) as i64);
        self.vm.grid[0][1] = Value::Int((angle_diff * 100.0) as i64);

        // 2. On Trace? (Triangle Inequality Check)
        let mut min_trace_dist = f64::MAX;
        for &(p1_idx, p2_idx) in &circuit.traces {
            let p1 = circuit.pads[p1_idx];
            let p2 = circuit.pads[p2_idx];
            let d_ab = hyperbolic_dist(p1, p2);
            let d_ap = hyperbolic_dist(p1, self.pos);
            let d_pb = hyperbolic_dist(self.pos, p2);
            let excess = (d_ap + d_pb) - d_ab;
            if excess < min_trace_dist {
                min_trace_dist = excess;
            }
        }

        let on_trace = min_trace_dist < 0.02; // Tolerance
        self.vm.grid[0][2] = Value::Int(if on_trace { 1 } else { 0 });

        // --- BRAIN ---
        self.vm.step();

        // --- ACTUATORS (Output from Grid) ---
        // Turn (15, 0)
        if let Value::Int(turn) = self.vm.grid[15][0] {
             self.heading += (turn as f64 * 0.1).to_radians();
             // Decay
             self.vm.grid[15][0] = Value::Int(turn / 2);
        }
        // Speed (15, 1)
        let speed = if let Value::Int(s) = self.vm.grid[15][1] {
            (s as f64 * 0.005).clamp(0.0, 0.02)
        } else { 0.0 };

        // Move
        // Step vector in local frame
        let step = Complex::from_polar(speed, self.heading);
        // mobius_add(step, pos) applies the translation 'pos' to the vector 'step'
        // Effectively moving from 'pos' by 'step'.
        self.pos = mobius_add(step, self.pos);

        // Check pad collision (arrival)
        if dist < 0.05 {
            // Arrived!
            self.vm.energy = self.vm.energy.saturating_add(50).min(100);
            self.target_idx = None; // Pick new target next frame
        }

        if !on_trace {
             self.vm.energy = self.vm.energy.saturating_sub(1);
        }
    }
}
