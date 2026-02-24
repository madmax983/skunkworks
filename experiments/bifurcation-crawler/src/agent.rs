use macroquad::prelude::*;
use chimera_lang::prelude::*;
use physics_pbd::{PbdSystem};

pub struct Crawler {
    pub vm: ChimeraVM,
    pub particle_indices: Vec<usize>,
    pub actuator_indices: Vec<usize>,
    pub color: Color,
    pub energy: f32,
    pub age: f32,
}

impl Crawler {
    pub fn new(pos: Vec2, system: &mut PbdSystem, dna: Dna) -> Self {
        let mut particle_indices = Vec::new();
        let mut actuator_indices = Vec::new();

        // Create a simple worm: 4 segments (5 particles)
        // Horizontal arrangement
        for i in 0..5 {
            let p_pos = vec3(pos.x + i as f32 * 0.05, pos.y, 0.0);
            // Mass 1.0 for head, slightly less for tail? Uniform is fine.
            let idx = system.add_particle(p_pos, 1.0);
            particle_indices.push(idx);
        }

        // Add Actuators (Muscles) between segments
        for i in 0..4 {
            let p1 = particle_indices[i];
            let p2 = particle_indices[i+1];
            // Min/Max/Stiffness
            system.add_actuator_constraint(p1, p2, 0.02, 0.10, 0.8);
            let c_idx = system.constraints.len() - 1;
            actuator_indices.push(c_idx);
        }

        // Add Struts for stability? Not needed for a chain.
        // But maybe a "skin" constraint?

        Crawler {
            vm: ChimeraVM::new(dna),
            particle_indices,
            actuator_indices,
            color: WHITE,
            energy: 100.0,
            age: 0.0,
        }
    }

    pub fn update(&mut self, system: &mut PbdSystem, chaos_factor: f32) {
        self.age += 0.016;

        // Input to VM:
        // 1. Current Chaos Level (0-100)
        // 2. Internal Energy (0-100)
        // 3. Average Strain (0-100)

        let strain = self.calculate_strain(system);

        self.vm.stack.clear();
        self.vm.stack.push(Value::Int((chaos_factor * 100.0) as i64));
        self.vm.stack.push(Value::Int(self.energy as i64));
        self.vm.stack.push(Value::Int((strain * 100.0) as i64));

        // Run VM
        for _ in 0..50 {
            let _ = self.vm.step();
        }

        // Output: Muscle contractions
        // Expecting 4 values for 4 actuators? Or 1 value for all?
        // Let's pop up to 4 values.

        for (i, &c_idx) in self.actuator_indices.iter().enumerate() {
            let factor = if let Some(val) = self.vm.stack.pop() {
                 match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                }
            } else {
                0.5 + (self.age * 5.0 + i as f32).sin() * 0.5 // Default: Peristaltic wave
            };

            // Apply to constraint
             if let physics_pbd::Constraint::Actuator { factor: ref mut f, .. } = &mut system.constraints[c_idx] {
                *f = factor;
            }
        }

        // Color reflects health/energy
        self.color = if self.energy > 50.0 { GREEN } else { RED };
        self.color.a = (self.energy / 100.0).clamp(0.2, 1.0);
    }

    fn calculate_strain(&self, system: &PbdSystem) -> f32 {
        let mut total = 0.0;
        for &c_idx in &self.actuator_indices {
             if let physics_pbd::Constraint::Actuator { p1, p2, max_len, .. } = &system.constraints[c_idx] {
                let pos1 = system.particles[*p1].pos;
                let pos2 = system.particles[*p2].pos;
                total += pos1.distance(pos2) / max_len;
            }
        }
        total / self.actuator_indices.len() as f32
    }
}
