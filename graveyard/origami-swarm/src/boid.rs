use ::rand::Rng;
use macroquad::prelude::*;
use neuro_sim::Network;
use physics_pbd::{Constraint, PbdSystem};

const SIM_STEPS: usize = 2;

#[derive(Clone)]
pub struct OrigamiBoid {
    pub id: usize,
    pub system: PbdSystem,
    pub brain: Network,
    pub center_of_mass: Vec3,
    pub velocity: Vec3,
    pub phase: f32,
    pub pacemaker_neuron: usize,
    pub actuator_idx: usize,
    pub mesh_indices: Vec<u16>,

    // Flocking parameters (DNA)
    pub max_speed: f32,
    pub max_force: f32,
    pub natural_freq: f32,
}

impl OrigamiBoid {
    pub fn new(id: usize, pos: Vec3) -> Self {
        let mut rng = ::rand::thread_rng();

        // 1. Create Physics Body (Simple Bird/Butterfly)
        // 4 Particles:
        // 0: Head
        // 1: Tail
        // 2: Left Wing Tip
        // 3: Right Wing Tip
        let mut system = PbdSystem::new();
        let scale = 1.0;

        // Relative positions
        let p_head = pos + vec3(0.0, 0.0, 1.0) * scale;
        let p_tail = pos + vec3(0.0, 0.0, -1.0) * scale;
        let p_left = pos + vec3(-1.5, 0.0, 0.0) * scale;
        let p_right = pos + vec3(1.5, 0.0, 0.0) * scale;

        let i_head = system.add_particle(p_head, 1.0);
        let i_tail = system.add_particle(p_tail, 1.0);
        let i_left = system.add_particle(p_left, 1.0);
        let i_right = system.add_particle(p_right, 1.0);

        // Structural Constraints
        let stiffness = 0.8;
        // Spine
        system.add_distance_constraint(i_head, i_tail, stiffness);
        // Left Wing Triangle
        system.add_distance_constraint(i_head, i_left, stiffness);
        system.add_distance_constraint(i_tail, i_left, stiffness);
        // Right Wing Triangle
        system.add_distance_constraint(i_head, i_right, stiffness);
        system.add_distance_constraint(i_tail, i_right, stiffness);

        // Actuator: Connect Left and Right Wing Tips
        // Contracting this pulls wings UP (dihedral)
        // Expanding pushes wings DOWN (anhedral)
        let dist = p_left.distance(p_right);
        system.add_actuator_constraint(i_left, i_right, dist * 0.4, dist * 1.2, 0.5);
        let actuator_idx = system.constraints.len() - 1;

        // Mesh Indices (2 Triangles)
        // Top side
        let mesh_indices = vec![
            i_head as u16,
            i_tail as u16,
            i_left as u16,
            i_head as u16,
            i_right as u16,
            i_tail as u16,
            // Bottom side (reverse winding)
            i_head as u16,
            i_left as u16,
            i_tail as u16,
            i_head as u16,
            i_tail as u16,
            i_right as u16,
        ];

        // 2. Create Brain (CPG)
        let mut brain = Network::new();
        // Simple Oscillator: 2 neurons (Excitatory + Inhibitory loop?)
        // Or just one self-exciting neuron with refractoriness?
        // Let's do a simple 2-neuron CPG: Pacemaker -> Motor
        let pacemaker = brain.add_neuron();
        let motor = brain.add_neuron();

        // Pacemaker excites itself to keep firing?
        // Or we drive it with the Phase variable.
        // Let's drive it with Phase.

        // Motor controls actuator
        brain.add_synapse(pacemaker, motor, 20.0);

        Self {
            id,
            system,
            brain,
            center_of_mass: pos,
            velocity: vec3(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize()
                * 0.5,
            phase: rng.gen::<f32>(),
            pacemaker_neuron: pacemaker,
            actuator_idx,
            mesh_indices,
            max_speed: 0.8,
            max_force: 0.05,
            natural_freq: 0.01 + rng.gen::<f32>() * 0.01,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        // Apply force to all particles based on mass
        // F = ma -> a = F/m
        // We just add velocity directly for PBD simplifications usually, but here we apply external force
        // But PBD step resets velocity?
        // In our PBD implementation: `p.vel += Vec3::ZERO * dt`.
        // We should add this force to the particles' velocity.

        let num_particles = self.system.particles.len() as f32;
        let force_per_particle = force / num_particles; // distribute force

        for p in &mut self.system.particles {
            if p.inv_mass > 0.0 {
                // Apply acceleration: v += a * dt?
                // Since this force is applied once per frame (outside PBD step), we can just add it to velocity directly?
                // Or better: modify the PBD `step` to accept external force.
                // For now, let's just add to `vel`.
                p.vel += force_per_particle;
            }
        }
    }

    pub fn update_flocking(
        &mut self,
        neighbors: &[Vec3],
        neighbor_vels: &[Vec3],
        neighbor_phases: &[f32],
        bounds_min: Vec3,
        bounds_max: Vec3,
    ) {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;

        let mut count = 0;
        let view_radius = 20.0;
        let separate_radius = 10.0;

        let mut phase_nudge = 0.0;
        let coupling_strength = 0.005;

        for i in 0..neighbors.len() {
            let other_pos = neighbors[i];
            let dist_sq = self.center_of_mass.distance_squared(other_pos);

            if dist_sq > 0.0 && dist_sq < view_radius * view_radius {
                // Separation
                if dist_sq < separate_radius * separate_radius {
                    let diff =
                        (self.center_of_mass - other_pos).normalize_or_zero() / dist_sq.sqrt();
                    separation += diff;
                }

                // Alignment
                alignment += neighbor_vels[i];

                // Cohesion
                cohesion += other_pos;

                // Phase Coupling (Kuramoto-like)
                // Firefly sync: nudge phase if neighbor flashes (phase near 0 or 1)
                // Or simpler continuous Kuramoto: dtheta = omega + K * sum(sin(theta_j - theta_i))
                let phase_diff = neighbor_phases[i] - self.phase;
                // sin(2PI * diff)
                phase_nudge += (phase_diff * std::f32::consts::TAU).sin();

                count += 1;
            }
        }

        if count > 0 {
            let count_f = count as f32;

            // Separation
            if separation.length_squared() > 0.0 {
                separation = separation.normalize() * self.max_speed;
                separation -= self.velocity;
                separation = separation.clamp_length_max(self.max_force);
            }

            // Alignment
            alignment /= count_f;
            if alignment.length_squared() > 0.0 {
                alignment = alignment.normalize() * self.max_speed;
                alignment -= self.velocity;
                alignment = alignment.clamp_length_max(self.max_force);
            }

            // Cohesion
            cohesion /= count_f;
            let mut desired = cohesion - self.center_of_mass;
            if desired.length_squared() > 0.0 {
                desired = desired.normalize() * self.max_speed;
                desired -= self.velocity;
                desired = desired.clamp_length_max(self.max_force);
            }
            cohesion = desired;

            // Apply forces
            let total_force = separation * 1.5 + alignment * 1.0 + cohesion * 1.0;
            self.apply_force(total_force);

            // Apply Phase Nudge
            self.phase += phase_nudge * coupling_strength / count_f;
        }

        // Boundary Wrap
        let size = bounds_max - bounds_min;
        let mut center_shift = Vec3::ZERO;

        if self.center_of_mass.x < bounds_min.x {
            center_shift.x += size.x;
        }
        if self.center_of_mass.x > bounds_max.x {
            center_shift.x -= size.x;
        }
        if self.center_of_mass.y < bounds_min.y {
            center_shift.y += size.y;
        }
        if self.center_of_mass.y > bounds_max.y {
            center_shift.y -= size.y;
        }
        if self.center_of_mass.z < bounds_min.z {
            center_shift.z += size.z;
        }
        if self.center_of_mass.z > bounds_max.z {
            center_shift.z -= size.z;
        }

        if center_shift != Vec3::ZERO {
            // Teleport particles
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
            // Fire pacemaker!
            // Inject current
        }

        // 2. Drive Brain
        let mut inputs = vec![0.0; self.brain.neurons.len()];
        // If phase is at peak (0.0 or 1.0), stimulate pacemaker
        if self.phase < 0.1 {
            inputs[self.pacemaker_neuron] = 20.0;
        }

        self.brain.step(&inputs);

        // 3. Drive Muscles (Actuators)
        // If motor neuron spikes, contract wings (flap up)
        // Else relax (flap down)

        // Actually we used `add_synapse(pacemaker, motor, ...)` so motor is idx 1.
        // Wait, `add_neuron` returns index.
        // self.pacemaker_neuron is 0. Motor is 1.

        let motor_idx = 1;
        let target_factor = if self.brain.is_spiking(motor_idx) {
            0.0 // Contract (Wings UP)
        } else {
            1.0 // Relax (Wings DOWN)
        };

        // Smoothly interpolate current factor
        if let Constraint::Actuator {
            factor,
            min_len,
            max_len,
            stiffness,
            p1,
            p2,
        } = self.system.constraints[self.actuator_idx]
        {
            let new_factor = factor + (target_factor - factor) * 0.1;
            self.system.constraints[self.actuator_idx] = Constraint::Actuator {
                p1,
                p2,
                min_len,
                max_len,
                stiffness,
                factor: new_factor,
            };
        }

        // 4. Step Physics
        // Dampen velocity for stability in air
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

            // Approximate bulk velocity
            let mut sum_vel = Vec3::ZERO;
            for p in &self.system.particles {
                sum_vel += p.vel;
            }
            self.velocity = sum_vel / count;
        }
    }

    pub fn draw(&self) {
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        // Color based on ID and Phase
        let hue = (self.id % 20) as f32 / 20.0;
        // Pulse brightness with phase
        let brightness = 0.5 + 0.5 * (self.phase * std::f32::consts::TAU).sin().abs();

        let color = Color::new(hue, 0.5, brightness, 1.0);
        let color_bytes: [u8; 4] = color.into();

        for i in &self.mesh_indices {
            let p = self.system.particles[*i as usize];
            let normal = vec4(0.0, 1.0, 0.0, 1.0); // Simplified normal
            mesh.vertices.push(Vertex {
                position: p.pos,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal,
            });
        }

        // Indices are just sequential because we pushed vertices in order of mesh_indices
        for i in 0..mesh.vertices.len() {
            mesh.indices.push(i as u16);
        }

        draw_mesh(&mesh);
    }
}
