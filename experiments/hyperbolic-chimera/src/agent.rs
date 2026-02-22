use chimera_lang::ast::Dna;
use chimera_lang::vm::{ChimeraVM, Value};
use num_complex::Complex;
use poincare_disk::{mobius_add, Point};
use ratatui::style::Color;
use std::f64::consts::PI;

pub struct HyperAgent {
    pub vm: ChimeraVM,
    pub pos: Point,
    pub color: Color,
}

impl HyperAgent {
    pub fn new(dna: Dna, start_pos: Point, color: Color) -> Self {
        Self {
            vm: ChimeraVM::new(dna),
            pos: start_pos,
            color,
        }
    }

    pub fn update(&mut self) {
        // 1. Inject Sensors (Grid[0][2] = Distance to center)
        // Distance in Poincaré disk is 2 * atanh(|z|)
        let r = self.pos.norm();
        let dist = 2.0 * r.atanh();
        let sensor_val = (dist * 10.0).clamp(0.0, 100.0) as i64;
        self.vm.grid[0][2] = Value::Int(sensor_val);

        // 2. Step VM
        // Run multiple ticks per frame for responsiveness
        for _ in 0..5 {
            if !self.vm.halted {
                self.vm.step();
            }
        }
        // Refuel slightly to prevent immediate death if they are moving
        if self.vm.energy < 50 {
            self.vm.energy += 1;
        }

        // 3. Read Actuators
        // Grid[0][0] = Angle (0-100 -> 0-2PI)
        // Grid[0][1] = Speed (0-100 -> 0.0-0.05)
        let angle_val = match &self.vm.grid[0][0] {
            Value::Int(v) => *v as f64,
            _ => 0.0,
        };
        let speed_val = match &self.vm.grid[0][1] {
            Value::Int(v) => *v as f64,
            _ => 0.0,
        };

        // Normalize
        let angle = (angle_val % 100.0) / 100.0 * 2.0 * PI;
        let speed = (speed_val.clamp(0.0, 100.0) / 100.0) * 0.05;

        // 4. Move
        // In local frame, move 'speed' in direction 'angle'
        // z_new = (z + w) / (1 + z*w_bar) where w is the step
        // But mobius_add(w, z) does this.
        let step = Complex::from_polar(speed, angle);

        // Apply movement relative to current position?
        // Or is "Angle" global?
        // If we use mobius_add(step, pos), 'step' is a translation from origin.
        // This effectively means 'angle' is relative to the "origin-ward" direction if we were to rotate the disk.
        // Actually mobius_add is non-commutative.
        // mobius_add(a, b) = (a+b)/(1+ab_bar).
        // If we want "agent moves in direction angle relative to itself", we need to handle rotation.
        // But for simplicity, let's treat Angle as a global direction on the disk (which is weird in hyperbolic space but works for basic movement).

        // Wait, if I am at P, and I want to move, I should transform a small step dP at origin to P.
        // Or simpler: just use mobius_add.
        self.pos = mobius_add(step, self.pos);

        // Boundary check (Poincaré disk must be |z| < 1)
        if self.pos.norm() >= 0.99 {
            // Bounce or clamp?
            // Let's clamp magnitude
            let arg = self.pos.arg();
            self.pos = Complex::from_polar(0.98, arg);
        }
    }
}
