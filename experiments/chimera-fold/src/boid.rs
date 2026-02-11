use crate::pbd::{Constraint, PbdSystem};
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use ::rand::Rng;

const SIM_STEPS: usize = 2;

#[derive(Clone)]
pub struct OrigamiBoid {
    pub id: usize,
    pub system: PbdSystem,
    pub vm: ChimeraVM,
    pub center_of_mass: Vec3,
    pub velocity: Vec3,
    pub phase: f32,
    pub actuator_idx: usize,
    pub mesh_indices: Vec<u16>,

    // Flocking parameters
    pub max_speed: f32,
    pub max_force: f32,
    pub natural_freq: f32,
}

impl OrigamiBoid {
    pub fn new(id: usize, pos: Vec3) -> Self {
        let mut rng = ::rand::thread_rng();

        // 1. Create Physics Body (Simple Bird/Butterfly)
        let mut system = PbdSystem::new();
        let scale = 1.0;

        let p_head = pos + vec3(0.0, 0.0, 1.0) * scale;
        let p_tail = pos + vec3(0.0, 0.0, -1.0) * scale;
        let p_left = pos + vec3(-1.5, 0.0, 0.0) * scale;
        let p_right = pos + vec3(1.5, 0.0, 0.0) * scale;

        let i_head = system.add_particle(p_head, 1.0);
        let i_tail = system.add_particle(p_tail, 1.0);
        let i_left = system.add_particle(p_left, 1.0);
        let i_right = system.add_particle(p_right, 1.0);

        let stiffness = 0.8;
        system.add_distance_constraint(i_head, i_tail, stiffness);
        system.add_distance_constraint(i_head, i_left, stiffness);
        system.add_distance_constraint(i_tail, i_left, stiffness);
        system.add_distance_constraint(i_head, i_right, stiffness);
        system.add_distance_constraint(i_tail, i_right, stiffness);

        let dist = p_left.distance(p_right);
        system.add_actuator_constraint(i_left, i_right, dist * 0.4, dist * 1.2, 0.5);
        let actuator_idx = system.constraints.len() - 1;

        let mesh_indices = vec![
            i_head as u16, i_tail as u16, i_left as u16,
            i_head as u16, i_right as u16, i_tail as u16,
            i_head as u16, i_left as u16, i_tail as u16,
            i_head as u16, i_tail as u16, i_right as u16,
        ];

        // 2. Create Brain (ChimeraVM)
        // Default DNA: Reads Phase (0,0) and writes to Actuator (1,0)
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // x=0
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // y=0
            Gene { op: OpCode::GRead, args: vec![] }, // Stack: [Phase]
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // x=0
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // y=1
            Gene { op: OpCode::GWrite, args: vec![] }, // Write Phase to Actuator
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Loop
        ];
        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 10000; // High energy for longevity

        Self {
            id,
            system,
            vm,
            center_of_mass: pos,
            velocity: vec3(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ).normalize() * 0.5,
            phase: rng.gen::<f32>(),
            actuator_idx,
            mesh_indices,
            max_speed: 0.8,
            max_force: 0.05,
            natural_freq: 0.01 + rng.gen::<f32>() * 0.01,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        let num_particles = self.system.particles.len() as f32;
        let force_per_particle = force / num_particles;
        for p in &mut self.system.particles {
            if p.inv_mass > 0.0 {
                p.vel += force_per_particle;
            }
        }
    }

    pub fn update_flocking(
        &mut self,
        neighbors: &[Vec3],
        neighbor_vels: &[Vec3],
        _neighbor_phases: &[f32],
        bounds_min: Vec3,
        bounds_max: Vec3,
    ) {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;

        let mut count = 0;
        let view_radius = 20.0;
        let separate_radius = 10.0;

        for i in 0..neighbors.len() {
            let other_pos = neighbors[i];
            let dist_sq = self.center_of_mass.distance_squared(other_pos);

            if dist_sq > 0.0 && dist_sq < view_radius * view_radius {
                // Separation
                if dist_sq < separate_radius * separate_radius {
                    let diff = (self.center_of_mass - other_pos).normalize_or_zero() / dist_sq.sqrt();
                    separation += diff;
                }
                // Alignment
                alignment += neighbor_vels[i];
                // Cohesion
                cohesion += other_pos;
                count += 1;
            }
        }

        if count > 0 {
            let count_f = count as f32;

            if separation.length_squared() > 0.0 {
                separation = separation.normalize() * self.max_speed;
                separation -= self.velocity;
                separation = separation.clamp_length_max(self.max_force);
            }

            alignment /= count_f;
            if alignment.length_squared() > 0.0 {
                alignment = alignment.normalize() * self.max_speed;
                alignment -= self.velocity;
                alignment = alignment.clamp_length_max(self.max_force);
            }

            cohesion /= count_f;
            let mut desired = cohesion - self.center_of_mass;
            if desired.length_squared() > 0.0 {
                desired = desired.normalize() * self.max_speed;
                desired -= self.velocity;
                desired = desired.clamp_length_max(self.max_force);
            }
            cohesion = desired;

            let total_force = separation * 1.5 + alignment * 1.0 + cohesion * 1.0;
            self.apply_force(total_force);

            // Write Neighbor Count to VM (Sensor 0,4)
            self.vm.grid[4][0] = Value::Int(count as i64);
        } else {
            self.vm.grid[4][0] = Value::Int(0);
        }

        // Boundary Wrap
        let size = bounds_max - bounds_min;
        let mut center_shift = Vec3::ZERO;

        if self.center_of_mass.x < bounds_min.x { center_shift.x += size.x; }
        if self.center_of_mass.x > bounds_max.x { center_shift.x -= size.x; }
        if self.center_of_mass.y < bounds_min.y { center_shift.y += size.y; }
        if self.center_of_mass.y > bounds_max.y { center_shift.y -= size.y; }
        if self.center_of_mass.z < bounds_min.z { center_shift.z += size.z; }
        if self.center_of_mass.z > bounds_max.z { center_shift.z -= size.z; }

        if center_shift != Vec3::ZERO {
            for p in &mut self.system.particles {
                p.pos += center_shift;
                p.prev_pos += center_shift;
            }
        }
    }

    pub fn update_internal(&mut self, dt: f32) {
        // 1. Update Phase
        self.phase += self.natural_freq;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // 2. Drive Brain
        let phase_int = (self.phase * 100.0) as i64;
        self.vm.grid[0][0] = Value::Int(phase_int);

        let vx = (self.velocity.x * 100.0) as i64;
        let vy = (self.velocity.y * 100.0) as i64;
        let vz = (self.velocity.z * 100.0) as i64;
        self.vm.grid[1][0] = Value::Int(vx);
        self.vm.grid[2][0] = Value::Int(vy);
        self.vm.grid[3][0] = Value::Int(vz);

        for _ in 0..10 {
            self.vm.step();
        }

        // 3. Read Actuators (1,0)
        let actuator_val = match &self.vm.grid[0][1] {
             Value::Int(n) => *n as f32 / 100.0,
             _ => 0.5,
        };
        let actuator_factor = actuator_val.clamp(0.0, 1.0);

        // Update Constraint
        if let Some(Constraint::Actuator { factor, .. }) = self.system.constraints.get(self.actuator_idx) {
             let current_factor = *factor;
             let new_factor = current_factor + (actuator_factor - current_factor) * 0.1;

             if let Constraint::Actuator { factor, .. } = &mut self.system.constraints[self.actuator_idx] {
                 *factor = new_factor;
             }
        }

        // 4. Step Physics
        for p in &mut self.system.particles {
            p.vel *= 0.98;
        }
        self.system.step(dt, SIM_STEPS);

        // 5. Update Center of Mass
        let mut sum_pos = Vec3::ZERO;
        let mut count = 0.0;
        for p in &self.system.particles {
            if p.inv_mass > 0.0 {
                sum_pos += p.pos;
                count += 1.0;
            }
        }
        if count > 0.0 {
            self.center_of_mass = sum_pos / count;
            let mut sum_vel = Vec3::ZERO;
            for p in &self.system.particles {
                sum_vel += p.vel;
            }
            self.velocity = sum_vel / count;
        }
    }

    pub fn draw(&self) {
         let r = match &self.vm.grid[1][1] { Value::Int(n) => *n as f32 / 100.0, _ => (self.id % 20) as f32 / 20.0 };
         let g = match &self.vm.grid[2][1] { Value::Int(n) => *n as f32 / 100.0, _ => 0.5 };
         let b = match &self.vm.grid[3][1] { Value::Int(n) => *n as f32 / 100.0, _ => 0.5 + 0.5 * (self.phase * std::f32::consts::TAU).sin().abs() };

         let color = Color::new(r, g, b, 1.0);
         let color_bytes: [u8; 4] = color.into();

         let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in &self.mesh_indices {
            let p = self.system.particles[*i as usize];
            let normal = vec4(0.0, 1.0, 0.0, 1.0);
            mesh.vertices.push(Vertex {
                position: p.pos,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal,
            });
        }

        for i in 0..mesh.vertices.len() {
            mesh.indices.push(i as u16);
        }

        draw_mesh(&mesh);
    }
}
