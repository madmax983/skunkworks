use crate::neuro::{CPGNetwork, IzhikevichNeuron};
use crate::physics::{DistanceConstraint, PhysicsWorld, VerletPoint};
use ::rand::Rng;
use macroquad::prelude::*;

pub struct Lamprey {
    pub left_muscles: Vec<usize>, // Indices of constraints
    pub right_muscles: Vec<usize>,
    pub cpg: CPGNetwork,
    pub neuron_map: Vec<(usize, usize)>, // (Left Neuron Index, Right Neuron Index) per segment
    pub num_segments: usize,
    pub base_drive: f32,
}

impl Lamprey {
    pub fn new(world: &mut PhysicsWorld, start_pos: Vec2) -> Self {
        let num_segments = 20;
        let segment_length = 15.0;
        let width = 10.0;
        let mut rng = ::rand::thread_rng();

        let mut left_points = Vec::new();
        let mut right_points = Vec::new();

        // 1. Create Body (Ladder)
        for i in 0..=num_segments {
            let x = start_pos.x + i as f32 * segment_length;
            let y_top = start_pos.y - width / 2.0;
            let y_bot = start_pos.y + width / 2.0;

            let p_top = VerletPoint::new(x, y_top);
            let p_bot = VerletPoint::new(x, y_bot);

            left_points.push(world.add_point(p_top));
            right_points.push(world.add_point(p_bot));
        }

        // 2. Add Constraints
        let mut left_muscles = Vec::new();
        let mut right_muscles = Vec::new();

        for i in 0..num_segments {
            let l1 = left_points[i];
            let l2 = left_points[i + 1];
            let r1 = right_points[i];
            let r2 = right_points[i + 1];

            // Rungs (Rigid)
            world.add_constraint(DistanceConstraint::new(l1, r1, width));
            if i == num_segments - 1 {
                world.add_constraint(DistanceConstraint::new(l2, r2, width));
            }

            // Diagonals (Rigid for shear stability)
            let diag_len = (segment_length * segment_length + width * width).sqrt();
            world.add_constraint(DistanceConstraint::new(l1, r2, diag_len));
            world.add_constraint(DistanceConstraint::new(r1, l2, diag_len));

            // Muscles (Side rails)
            let l_constraint = DistanceConstraint::new(l1, l2, segment_length);
            let r_constraint = DistanceConstraint::new(r1, r2, segment_length);

            world.add_constraint(l_constraint);
            left_muscles.push(world.constraints.len() - 1);

            world.add_constraint(r_constraint);
            right_muscles.push(world.constraints.len() - 1);
        }

        // 3. Create CPG
        let mut cpg = CPGNetwork::new();
        let mut neuron_map = Vec::new();

        for _ in 0..num_segments {
            // Left Neuron
            let mut ln = IzhikevichNeuron::new_rs();
            // Right Neuron
            let mut rn = IzhikevichNeuron::new_rs();

            // Add some noise to initial state to break symmetry?
            ln.v = -65.0 + rng.gen_range(-5.0..5.0);
            rn.v = -65.0 + rng.gen_range(-5.0..5.0);

            let l_idx = cpg.add_neuron(ln);
            let r_idx = cpg.add_neuron(rn);

            neuron_map.push((l_idx, r_idx));
        }

        // 4. Connect CPG
        let weight_desc = 20.0;
        let weight_inhib = -25.0; // Strong inhibition
        let delay_inter = 2; // ticks

        for i in 0..num_segments {
            let (l, r) = neuron_map[i];

            // Cross Inhibition (Left <-> Right)
            cpg.add_synapse(l, r, weight_inhib, 0);
            cpg.add_synapse(r, l, weight_inhib, 0);

            // Descending Excitation
            if i < num_segments - 1 {
                let (next_l, next_r) = neuron_map[i + 1];

                // Ipsilateral coupling (Left -> Next Left)
                cpg.add_synapse(l, next_l, weight_desc, delay_inter);
                cpg.add_synapse(r, next_r, weight_desc, delay_inter);
            }
        }

        Self {
            left_muscles,
            right_muscles,
            cpg,
            neuron_map,
            num_segments,
            base_drive: 15.0,
        }
    }

    pub fn update(&mut self, dt: f32, world: &mut PhysicsWorld, drive_override: Option<f32>) {
        // Step CPG
        let drive = drive_override.unwrap_or(self.base_drive);
        let mut inputs = vec![0.0; self.cpg.neurons.len()];

        // Drive the first few segments to start the wave
        for i in 0..3 {
            let (l, r) = self.neuron_map[i];
            inputs[l] = drive;
            inputs[r] = drive;
        }

        // Weak global drive
        for i in 3..self.num_segments {
            let (l, r) = self.neuron_map[i];
            inputs[l] = drive * 0.8;
            inputs[r] = drive * 0.8;
        }

        // Izhikevich model uses ms time steps. dt is in seconds.
        // We need to sub-step the neural simulation for stability.
        // Target dt = 1.0 ms.
        let total_ms = dt * 1000.0;
        let steps = (total_ms / 1.0).ceil() as usize;
        let sub_dt = total_ms / steps as f32;

        for _ in 0..steps {
            self.cpg.step(sub_dt, &inputs);
        }

        // Map CPG to Muscles
        let contraction_strength = 0.2; // 20% shortening
        let base_len = 15.0;

        for i in 0..self.num_segments {
            let (l_idx, r_idx) = self.neuron_map[i];
            let l_neuron = &self.cpg.neurons[l_idx];
            let r_neuron = &self.cpg.neurons[r_idx];

            // Calculate activation
            let l_act = ((l_neuron.v + 50.0) / 20.0).clamp(0.0, 1.0);
            let r_act = ((r_neuron.v + 50.0) / 20.0).clamp(0.0, 1.0);

            let l_target = base_len * (1.0 - l_act * contraction_strength);
            let r_target = base_len * (1.0 - r_act * contraction_strength);

            // Update constraint lengths
            let l_muscle_idx = self.left_muscles[i];
            let r_muscle_idx = self.right_muscles[i];

            world.constraints[l_muscle_idx].length = l_target;
            world.constraints[r_muscle_idx].length = r_target;
        }
    }
}
