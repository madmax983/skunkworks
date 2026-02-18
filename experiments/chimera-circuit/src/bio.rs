use chimera_lang::prelude::*;
use macroquad::prelude::*;
use image::RgbaImage;

pub struct BioAgent {
    pub vm: ChimeraVM,
    pub pos: Vec2,
    pub dir: Vec2,
    pub target: Option<Vec2>,
    pub id: u64,
    pub generation: u64,
    pub fitness: f32,
}

impl BioAgent {
    pub fn new(pos: Vec2, dna: Dna) -> Self {
        let vm = ChimeraVM::new(dna);
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let dir = vec2(angle.cos(), angle.sin());

        Self {
            vm,
            pos,
            dir,
            target: None,
            id: rand::gen_range(0, 1_000_000_000),
            generation: 0,
            fitness: 0.0,
        }
    }

    pub fn update(&mut self, circuit: &RgbaImage, pads: &[(u32, u32)]) {
        if self.vm.halted {
            return;
        }

        // --- SENSORS (Input to Grid) ---

        // 1. Trace Sensor (0, 0)
        let x = self.pos.x as u32;
        let y = self.pos.y as u32;
        let on_trace = if x < circuit.width() && y < circuit.height() {
            let pixel = circuit.get_pixel(x, y);
            // Trace color is roughly Green (0, 170, 0), Pad is Gold (255, 215, 0)
            // BG is Dark Green (0, 68, 0)
            // We check if Green channel is high enough (> 100) or Red is high (Pad)
            pixel[1] > 100 || pixel[0] > 100
        } else {
            false
        };
        self.vm.grid[0][0] = Value::Int(if on_trace { 1 } else { 0 });

        // 2. Target Sensors (0, 1) & (0, 2)
        if let Some(target) = self.target {
            let to_target = target - self.pos;
            let dist = to_target.length();
            let angle_to_target = to_target.y.atan2(to_target.x);
            let current_angle = self.dir.y.atan2(self.dir.x);
            let mut diff = angle_to_target - current_angle;

            // Normalize angle diff to -PI..PI
            while diff > std::f32::consts::PI { diff -= 2.0 * std::f32::consts::PI; }
            while diff < -std::f32::consts::PI { diff += 2.0 * std::f32::consts::PI; }

            self.vm.grid[0][1] = Value::Int(dist as i64);
            self.vm.grid[0][2] = Value::Int((diff * 100.0) as i64); // Scaled angle
        } else {
            // Find nearest pad as default target logic if none set
            let mut min_dist_sq = f32::MAX;
            let mut nearest = None;
            for &(px, py) in pads {
                let p_vec = vec2(px as f32, py as f32);
                let d_sq = self.pos.distance_squared(p_vec);
                if d_sq < min_dist_sq && d_sq > 100.0 { // Don't target current pad
                     min_dist_sq = d_sq;
                     nearest = Some(p_vec);
                }
            }
            self.target = nearest;
            self.vm.grid[0][1] = Value::Int(9999);
            self.vm.grid[0][2] = Value::Int(0);
        }

        // --- BRAIN ---
        self.vm.step();

        // --- ACTUATORS (Output from Grid) ---

        // 1. Turn (15, 0) - Range -100 to 100
        if let Value::Int(turn_val) = self.vm.grid[15][0] {
             let turn_angle = (turn_val as f32 * 0.05).to_radians(); // Sensitivity
             let cos_a = turn_angle.cos();
             let sin_a = turn_angle.sin();
             let new_x = self.dir.x * cos_a - self.dir.y * sin_a;
             let new_y = self.dir.x * sin_a + self.dir.y * cos_a;
             self.dir = vec2(new_x, new_y).normalize();

             // Decay the signal
             self.vm.grid[15][0] = Value::Int(turn_val / 2);
        }

        // 2. Speed (15, 1) - Range 0 to 10
        let speed = if let Value::Int(s) = self.vm.grid[15][1] {
            (s as f32).clamp(0.0, 5.0)
        } else {
            0.0
        };

        // Move
        self.pos += self.dir * speed;

        // --- METABOLISM ---
        if !on_trace {
            self.vm.energy -= 2; // High penalty for off-road
        }

        // Check pad collision (Recharge)
        let x_u = self.pos.x as u32;
        let y_u = self.pos.y as u32;
        if x_u < circuit.width() && y_u < circuit.height() {
            let pixel = circuit.get_pixel(x_u, y_u);
            if pixel[0] > 200 && pixel[1] > 200 { // Gold-ish
                 self.vm.energy = self.vm.energy.saturating_add(5).min(100);
                 self.fitness += 1.0;
            }
        }
    }
}
