use macroquad::prelude::*;
use crate::network::Network;
use crate::heightmap::HeightMap;

const TERRAIN_SCALE: f32 = 5.0;
const MAP_WIDTH: f32 = 200.0;
const MAP_HEIGHT: f32 = 200.0;
const OFFSET_X: f32 = MAP_WIDTH / 2.0;
const OFFSET_Z: f32 = MAP_HEIGHT / 2.0;

pub struct Leg3D {
    pub base_offset: Vec3, // Offset from body center
    pub current_pos: Vec3, // World position of foot
    pub target_pos: Vec3,  // Target world position
    pub angle: f32,        // Swing angle (yaw relative to body)
    pub range: f32,        // Swing range
    pub flexor_idx: usize,
    pub extensor_idx: usize,
    pub lift: f32,         // 0.0 = on ground, 1.0 = fully lifted
}

impl Leg3D {
    pub fn new(base_offset: Vec3, flexor_idx: usize, extensor_idx: usize) -> Self {
        Self {
            base_offset,
            current_pos: Vec3::ZERO,
            target_pos: Vec3::ZERO,
            angle: 0.0,
            range: 0.5,
            flexor_idx,
            extensor_idx,
            lift: 0.0,
        }
    }
}

pub struct Crab3D {
    pub pos: Vec3,
    pub yaw: f32,
    pub velocity: Vec3,
    pub legs: Vec<Leg3D>,
    pub network: Network,
}

impl Crab3D {
    pub fn new(start_pos: Vec3) -> Self {
        let mut network = Network::new();
        let mut legs = Vec::new();

        // 6 legs x 2 neurons = 12 neurons
        for _ in 0..12 {
            network.add_neuron();
        }

        // Leg Configuration (Hexapod)
        // L1, L2, L3 (Left side)
        // R1, R2, R3 (Right side)
        let leg_dist_x = 3.0;
        let leg_dist_z = 3.0;

        let offsets = vec![
            vec3(-leg_dist_x, 0.0, leg_dist_z),  // L1
            vec3(-leg_dist_x, 0.0, 0.0),         // L2
            vec3(-leg_dist_x, 0.0, -leg_dist_z), // L3
            vec3(leg_dist_x, 0.0, leg_dist_z),   // R1
            vec3(leg_dist_x, 0.0, 0.0),          // R2
            vec3(leg_dist_x, 0.0, -leg_dist_z),  // R3
        ];

        for (i, offset) in offsets.iter().enumerate() {
            let f = i * 2;
            let e = i * 2 + 1;
            let mut leg = Leg3D::new(*offset, f, e);
            // Initialize foot position
            leg.current_pos = start_pos + *offset;
            leg.target_pos = start_pos + *offset;
            legs.push(leg);

            // CPG / Reflexes
            network.add_synapse(f, e, -10.0);
            network.add_synapse(e, f, -10.0);
        }

        // Tripod Gait Topology
        let neighbor_pairs = vec![
            (0, 1), (1, 2), // Left chain
            (3, 4), (4, 5), // Right chain
            (0, 3), (1, 4), (2, 5), // Cross links
        ];

        for (a, b) in neighbor_pairs {
            let af = a * 2;
            let bf = b * 2;
            network.add_synapse(af, bf, -5.0);
            network.add_synapse(bf, af, -5.0);
        }

        Self {
            pos: start_pos,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            legs,
            network,
        }
    }

    pub fn update(&mut self, dt: f32, drive: f32, heightmap: &HeightMap) {
        // 1. Update Network
        let mut inputs = vec![0.0; 12];
        for i in 0..12 {
            inputs[i] = drive + rand::gen_range(-1.0, 1.0);
        }
        self.network.step(&inputs);

        // 2. Update Legs
        let body_rot = Quat::from_rotation_y(self.yaw);
        let forward = body_rot * Vec3::Z;
        let right = body_rot * Vec3::X;

        // Determine average movement vector from leg swings
        let mut move_vec = Vec3::ZERO;
        let mut rot_torque = 0.0;

        for leg in &mut self.legs {
            let f_spike = self.network.is_spiking(leg.flexor_idx);
            let e_spike = self.network.is_spiking(leg.extensor_idx);

            // Swing Logic
            if f_spike {
                leg.angle += 2.0 * dt; // Swing forward
                leg.lift = (leg.lift + 5.0 * dt).min(1.0);
            } else if e_spike {
                leg.angle -= 2.0 * dt; // Swing backward (Stance)
                leg.lift = (leg.lift - 5.0 * dt).max(0.0);
            } else {
                // Relax to 0
                leg.angle *= 0.95;
                leg.lift *= 0.95;
            }

            // Calculate Target Pos relative to body
            // Base pos in world
            let leg_base_world = self.pos + body_rot * leg.base_offset;

            // Swing direction (Unused for simple swing, kept for potential complex IK)
            let _swing_dir = (forward * leg.angle.sin() + right * leg.angle.cos()).normalize_or_zero();

            // Just use simple forward/back swing for now
            // Forward is Z? In OpenGL/Macroquad:
            // Assuming Z is forward for crab logic?
            // Actually, let's say Yaw 0 points along Z.

            let swing_offset = forward * leg.angle * 3.0;
            let lateral_offset = right * leg.base_offset.x.signum() * 5.0; // Reach out

            leg.target_pos = leg_base_world + lateral_offset + swing_offset;

            // Ground Clamping
            let grid_x = (leg.target_pos.x / TERRAIN_SCALE + OFFSET_X).round() as i32;
            let grid_z = (leg.target_pos.z / TERRAIN_SCALE + OFFSET_Z).round() as i32;

            let ground_y = if grid_x >= 0 && grid_x < MAP_WIDTH as i32 && grid_z >= 0 && grid_z < MAP_HEIGHT as i32 {
                heightmap.get(grid_x as u32, grid_z as u32) * 2.0 // Scale match
            } else {
                0.0
            };

            leg.target_pos.y = ground_y;

            // Interpolate Current Pos
            let lift_height = leg.lift * 2.0;
            leg.current_pos.x += (leg.target_pos.x - leg.current_pos.x) * 10.0 * dt;
            leg.current_pos.z += (leg.target_pos.z - leg.current_pos.z) * 10.0 * dt;
            leg.current_pos.y += ((leg.target_pos.y + lift_height) - leg.current_pos.y) * 10.0 * dt;

            // Propulsion: If leg is on ground (lift < 0.1) and moving back (angle decreasing), push body forward
            if leg.lift < 0.1 && e_spike {
                move_vec += forward * 5.0 * dt;
                // Add some rotation if legs are asymmetrical?
                rot_torque += leg.base_offset.x.signum() * 0.5 * dt;
            }
        }

        // 3. Move Body
        self.velocity += move_vec;
        self.velocity *= 0.9; // Friction
        self.yaw += rot_torque;

        self.pos += self.velocity * dt;

        // 4. Body Height/Tilt Adjustment
        // Average leg Y
        let avg_leg_y: f32 = self.legs.iter().map(|l| l.current_pos.y).sum::<f32>() / 6.0;
        let target_body_y = avg_leg_y + 5.0; // Body floats above legs
        self.pos.y += (target_body_y - self.pos.y) * 5.0 * dt;

        // Tilt could be calculated from leg plane, simplified for now
    }
}
