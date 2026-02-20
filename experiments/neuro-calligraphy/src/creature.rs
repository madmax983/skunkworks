use neuro_sim::Network;
use crate::physics::{DistanceConstraint, PhysicsWorld, VerletPoint};
use ::rand::Rng;
use macroquad::prelude::*;

pub struct ContourCreature {
    pub left_points: Vec<usize>,
    pub right_points: Vec<usize>,
    pub cpg: Network,
    pub neuron_map: Vec<(usize, usize)>, // (Left Neuron, Right Neuron) per segment
    pub muscles: Vec<(usize, usize)>,    // (Left Constraint, Right Constraint) per segment
    pub segment_lengths: Vec<f32>,
    pub base_drive: f32,
}

impl ContourCreature {
    pub fn new(world: &mut PhysicsWorld, contour: &[Vec2], thickness: f32) -> Self {
        let n = contour.len();
        let mut left_indices = Vec::new();
        let mut right_indices = Vec::new();
        let mut segment_lengths = Vec::new();

        // 1. Create Points (Ribbon Extrusion)
        for i in 0..n {
            let p = contour[i];
            let prev = contour[(i + n - 1) % n];
            let next = contour[(i + 1) % n];

            // Calculate tangent
            let tangent = (next - prev).normalize_or_zero();
            // Normal (rotate 90 deg)
            let normal = vec2(-tangent.y, tangent.x);

            let p_left = p + normal * (thickness / 2.0);
            let p_right = p - normal * (thickness / 2.0);

            left_indices.push(world.add_point(VerletPoint::new(p_left.x, p_left.y)));
            right_indices.push(world.add_point(VerletPoint::new(p_right.x, p_right.y)));
        }

        // 2. Add Constraints
        // Rungs (Rigid)
        for i in 0..n {
            let l = left_indices[i];
            let r = right_indices[i];
            world.add_constraint(DistanceConstraint::new(l, r, thickness));
        }

        // Longitudinal Muscles & Diagonals
        let mut muscle_indices = Vec::new();
        for i in 0..n {
            let next_i = (i + 1) % n;

            let l1 = left_indices[i];
            let l2 = left_indices[next_i];
            let r1 = right_indices[i];
            let r2 = right_indices[next_i];

            let segment_len = (contour[next_i] - contour[i]).length();
            segment_lengths.push(segment_len);

            // Muscles (Side rails) - Mutable length
            let l_c = DistanceConstraint::new(l1, l2, segment_len);
            let r_c = DistanceConstraint::new(r1, r2, segment_len);

            world.add_constraint(l_c);
            let l_idx = world.constraints.len() - 1;

            world.add_constraint(r_c);
            let r_idx = world.constraints.len() - 1;

            muscle_indices.push((l_idx, r_idx));

            // Diagonals (Rigid) for shear stability
            // Use average length or calculate exact
            // We approximate diagonal
            let diag_len = (segment_len.powi(2) + thickness.powi(2)).sqrt();
            world.add_constraint(DistanceConstraint::new(l1, r2, diag_len));
            world.add_constraint(DistanceConstraint::new(r1, l2, diag_len));
        }

        // 3. Create CPG (Ring Network)
        let mut cpg = Network::new();
        let mut neuron_map = Vec::new();
        let mut rng = ::rand::thread_rng();

        for _ in 0..n {
            // Add Default Neurons (Regular Spiking)
            let l_idx = cpg.add_neuron();
            let r_idx = cpg.add_neuron();

            // Randomize initial voltage
            cpg.neurons[l_idx].v = -65.0 + rng.gen_range(-5.0..5.0);
            cpg.neurons[r_idx].v = -65.0 + rng.gen_range(-5.0..5.0);

            neuron_map.push((l_idx, r_idx));
        }

        // Connect CPG
        let weight_inhib = -20.0;
        let weight_exc = 15.0;
        let delay = 2;

        for i in 0..n {
            let (l, r) = neuron_map[i];
            let next_i = (i + 1) % n;
            let (next_l, next_r) = neuron_map[next_i];

            // Cross Inhibition (Delay 0 = immediate/next step)
            cpg.add_synapse_with_delay(l, r, weight_inhib, 0);
            cpg.add_synapse_with_delay(r, l, weight_inhib, 0);

            // Forward Excitation (Wave propagation)
            cpg.add_synapse_with_delay(l, next_l, weight_exc, delay);
            cpg.add_synapse_with_delay(r, next_r, weight_exc, delay);
        }

        Self {
            left_points: left_indices,
            right_points: right_indices,
            cpg,
            neuron_map,
            muscles: muscle_indices,
            segment_lengths,
            base_drive: 10.0,
        }
    }

    pub fn update(&mut self, dt: f32, world: &mut PhysicsWorld) {
        // Step CPG
        let drive = self.base_drive;
        let inputs: Vec<f32> = (0..self.cpg.neurons.len()).map(|_| drive).collect();

        let total_ms = dt * 1000.0;
        let steps = (total_ms / 1.0).ceil() as usize; // 1ms steps
        // let sub_dt = total_ms / steps as f32; // Unused as neuro-sim assumes 1.0

        for _ in 0..steps {
            self.cpg.step(&inputs);
        }

        // Update Muscles
        let contraction_strength = 0.3; // 30% shortening

        for i in 0..self.muscles.len() {
            let (l_muscle_idx, r_muscle_idx) = self.muscles[i];
            let base_len = self.segment_lengths[i];

            let (l_neuron_idx, r_neuron_idx) = self.neuron_map[i];
            let l_neuron = &self.cpg.neurons[l_neuron_idx];
            let r_neuron = &self.cpg.neurons[r_neuron_idx];

            // Normalize voltage (-65 to 30) to 0..1 activation
            let l_act = ((l_neuron.v + 65.0) / 95.0).clamp(0.0, 1.0);
            let r_act = ((r_neuron.v + 65.0) / 95.0).clamp(0.0, 1.0);

            // Contract based on activation
            let l_target = base_len * (1.0 - l_act * contraction_strength);
            let r_target = base_len * (1.0 - r_act * contraction_strength);

            world.constraints[l_muscle_idx].length = l_target;
            world.constraints[r_muscle_idx].length = r_target;
        }
    }

    pub fn draw(&self, world: &PhysicsWorld) {
        let n = self.left_points.len();
        for i in 0..n {
            let next_i = (i + 1) % n;
            let l1 = world.points[self.left_points[i]].pos;
            let l2 = world.points[self.left_points[next_i]].pos;
            let r1 = world.points[self.right_points[i]].pos;
            let r2 = world.points[self.right_points[next_i]].pos;

            // Interpolate color based on activation?
            // Just use fixed colors for now
            draw_triangle(l1, r1, r2, Color::new(0.8, 0.2, 0.2, 0.8));
            draw_triangle(l1, r2, l2, Color::new(0.2, 0.2, 0.8, 0.8));

            // Draw skeleton lines
            // draw_line(l1.x, l1.y, l2.x, l2.y, 1.0, WHITE);
            // draw_line(r1.x, r1.y, r2.x, r2.y, 1.0, WHITE);
            // draw_line(l1.x, l1.y, r1.x, r1.y, 1.0, WHITE);
        }
    }
}
