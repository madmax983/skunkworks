use crate::brain::{Brain, N_INTER, N_SENSORY};
use macroquad::prelude::*;

const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.1;
const SENSOR_RADIUS: f32 = 100.0;

#[derive(Clone, Copy)]
pub struct NeighborInfo {
    pub dist: f32,
    pub angle: f32, // -PI to PI
}

pub struct Agent {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub brain: Brain,
    pub color: Color,
}

impl Agent {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
            acc: Vec2::ZERO,
            brain: Brain::new(),
            color: WHITE,
        }
    }

    pub fn update(&mut self, dt: f32, neighbors: &[NeighborInfo], screen_w: f32, screen_h: f32) {
        // 1. Process Sensors
        // Find nearest neighbor
        let mut min_dist = SENSOR_RADIUS;
        let mut nearest_angle = 0.0;
        let mut found_neighbor = false;

        for n in neighbors {
            if n.dist < min_dist && n.dist > 0.0 {
                min_dist = n.dist;
                nearest_angle = n.angle;
                found_neighbor = true;
            }
        }

        // Inputs to Brain
        let mut inputs = vec![0.0; N_SENSORY];
        if found_neighbor {
            // Input 0: Proximity (1.0 = very close, 0.0 = far)
            inputs[0] = 1.0 - (min_dist / SENSOR_RADIUS).clamp(0.0, 1.0);

            // Input 1: Angle (normalize -PI..PI to -1..1)
            inputs[1] = nearest_angle / std::f32::consts::PI;
        }

        // Input 2: Current Speed (normalized)
        inputs[2] = (self.vel.length() / MAX_SPEED).clamp(0.0, 1.0);

        // Input 3: Random Noise (Spontaneous activity)
        inputs[3] = rand::gen_range(0.0, 1.0);

        // 2. Update Brain
        // dt is in seconds, brain expects roughly ms-scale or generic units.
        // Let's pass dt * 100.0 for speed.
        self.brain.update(dt * 100.0, &inputs);

        // 3. Process Actuators
        // Use traces (calcium/activity level) for smooth movement
        let motor_start = N_SENSORY + N_INTER;
        let t1 = self.brain.traces[motor_start]; // Thrust Trace
        let t2 = self.brain.traces[motor_start + 1]; // Turn Trace

        // Map trace (0.0 to ~2.0) to force
        let thrust = (t1 * 0.5).clamp(0.0, 1.0) * MAX_FORCE;

        // Turn: t2 determines turn direction?
        // Or maybe t2 is Left, t3 is Right?
        // I only have 2 motor neurons.
        // Let's make t2 drive turn in one direction (Left)
        // And maybe lack of t2 means straight?
        // Or map t2 to [-1, 1]? Trace is always positive.
        // Let's bias it: (t2 - 0.5) * 2.0?
        // Or better: Use t2 for Turn Right, and define logic:
        // if t2 > threshold -> Turn.
        // But what turns Left?
        // Maybe Neuron 1 is Thrust, Neuron 2 is Turn Left, Neuron 3 is Turn Right?
        // I defined N_MOTOR = 2.
        // Let's stick to: t1 = Thrust, t2 = Torque.
        // But trace is positive.
        // So t2 only turns one way?
        // That's fine. Boids can loop.
        // Or: t2 > 0.5 -> Turn Right.

        let turn = (t2.clamp(0.0, 2.0) - 0.5) * MAX_FORCE * 2.0;

        // Apply Forces
        // Forward force
        let forward = if self.vel.length() > 0.001 {
            self.vel.normalize()
        } else {
            vec2(1.0, 0.0)
        };

        self.acc += forward * thrust;

        // Turning force (Perpendicular)
        let right = vec2(-forward.y, forward.x);
        self.acc += right * turn * MAX_FORCE;

        // Physics Update
        self.vel += self.acc;
        if self.vel.length() > MAX_SPEED {
            self.vel = self.vel.normalize() * MAX_SPEED;
        }
        self.pos += self.vel;
        self.acc = Vec2::ZERO;

        // Wrap around screen
        if self.pos.x < 0.0 {
            self.pos.x = screen_w;
        }
        if self.pos.x > screen_w {
            self.pos.x = 0.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = screen_h;
        }
        if self.pos.y > screen_h {
            self.pos.y = 0.0;
        }

        // Update Color based on brain activity
        // Average voltage of all neurons
        let avg_v: f32 =
            self.brain.neurons.iter().map(|n| n.v).sum::<f32>() / self.brain.neurons.len() as f32;
        // Map -65 to 30 -> 0 to 1
        let excitement = ((avg_v + 65.0) / 95.0).clamp(0.0, 1.0);
        self.color = Color::new(1.0, 1.0 - excitement, 1.0 - excitement, 1.0); // White to Red
    }

    pub fn draw(&self) {
        let forward = if self.vel.length() > 0.001 {
            self.vel.normalize()
        } else {
            vec2(1.0, 0.0)
        };
        let right = vec2(-forward.y, forward.x);

        let tip = self.pos + forward * 10.0;
        let l_wing = self.pos - forward * 5.0 - right * 5.0;
        let r_wing = self.pos - forward * 5.0 + right * 5.0;

        draw_triangle(tip, l_wing, r_wing, self.color);
    }
}
