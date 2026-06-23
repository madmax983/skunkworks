use crate::attractor::{lorenz_velocity, LorenzParams};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct TurtleState {
    pub pos: Vec3,
    pub heading: Vec3,
    pub up: Vec3,
    pub length_scale: f32,
}

pub struct ChaoticTurtle {
    state: TurtleState,
    stack: Vec<TurtleState>,
    pub lines: Vec<(Vec3, Vec3, Color)>, // Start, End, Color
    params: LorenzParams,
    flow_influence: f32, // How much the attractor affects direction (0.0 to 1.0)
    step_size: f32,      // Simulation step size
}

impl ChaoticTurtle {
    pub fn new(start_pos: Vec3, params: LorenzParams) -> Self {
        Self {
            state: TurtleState {
                pos: start_pos,
                heading: vec3(0.0, 1.0, 0.0), // Up initially
                up: vec3(0.0, 0.0, 1.0),
                length_scale: 1.0,
            },
            stack: Vec::new(),
            lines: Vec::new(),
            params,
            flow_influence: 0.1,
            step_size: 0.01,
        }
    }

    pub fn set_flow_influence(&mut self, val: f32) {
        self.flow_influence = val;
    }

    pub fn forward(&mut self, distance: f32, color: Color) {
        let num_steps = (distance / self.step_size).ceil() as usize;
        let actual_step = distance / num_steps as f32;

        let mut current_pos = self.state.pos;

        for _ in 0..num_steps {
            // Calculate Lorenz velocity field at current position
            let flow = lorenz_velocity(current_pos, self.params).normalize_or_zero();

            // Blend heading with flow
            if flow.length_squared() > 0.0 {
                self.state.heading = self
                    .state
                    .heading
                    .lerp(flow, self.flow_influence)
                    .normalize();
                // Re-orthogonalize up vector
                let right = self.state.heading.cross(self.state.up).normalize_or_zero();
                if right.length_squared() > 0.0 {
                    self.state.up = right.cross(self.state.heading).normalize();
                }
            }

            let next_pos = current_pos + self.state.heading * actual_step;
            self.lines.push((current_pos, next_pos, color));
            current_pos = next_pos;
        }
        self.state.pos = current_pos;
    }

    pub fn turn(&mut self, angle_degrees: f32) {
        let rot = Quat::from_axis_angle(self.state.up, angle_degrees.to_radians());
        self.state.heading = rot.mul_vec3(self.state.heading);
    }

    pub fn pitch(&mut self, angle_degrees: f32) {
        let right = self.state.heading.cross(self.state.up).normalize();
        let rot = Quat::from_axis_angle(right, angle_degrees.to_radians());
        self.state.heading = rot.mul_vec3(self.state.heading);
        self.state.up = rot.mul_vec3(self.state.up);
    }

    pub fn roll(&mut self, angle_degrees: f32) {
        let rot = Quat::from_axis_angle(self.state.heading, angle_degrees.to_radians());
        self.state.up = rot.mul_vec3(self.state.up);
    }

    pub fn push(&mut self) {
        self.stack.push(self.state);
        self.state.length_scale *= 0.9; // Decay length slightly on branches
    }

    pub fn pop(&mut self) {
        if let Some(s) = self.stack.pop() {
            self.state = s;
        }
    }
}
