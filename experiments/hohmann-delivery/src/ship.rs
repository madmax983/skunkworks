use crate::guidance::{calculate_hohmann_transfer, TransferWindow};
use crate::physics::Body;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum ShipState {
    Docked {
        body_index: usize,
    },
    WaitingForWindow {
        body_index: usize,
        target_index: usize,
        plan: TransferWindow,
    },
    InTransit {
        target_index: usize,
        plan: TransferWindow,
    },
    Arrived {
        body_index: usize,
    },
}

#[derive(Debug, Clone)]
pub enum ShipEvent {
    None,
    Launched,
    Arrived(usize),
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: ShipState,
    pub fuel: f32,
    pub cargo: u32,
    pub color: Color,
}

impl Ship {
    pub fn new(body_index: usize, body: &Body) -> Self {
        Self {
            pos: body.pos,
            vel: body.vel,
            state: ShipState::Docked { body_index },
            fuel: 1000.0,
            cargo: 0,
            color: WHITE,
        }
    }

    pub fn schedule_transfer(&mut self, bodies: &[Body], target_index: usize, gm: f32) {
        // Only schedule if docked
        let current_body_index = match self.state {
            ShipState::Docked { body_index } => Some(body_index),
            ShipState::Arrived { body_index } => Some(body_index),
            _ => None,
        };

        if let Some(body_index) = current_body_index {
            if let Some(plan) =
                calculate_hohmann_transfer(&bodies[body_index], &bodies[target_index], gm)
            {
                self.state = ShipState::WaitingForWindow {
                    body_index,
                    target_index,
                    plan,
                };
            }
        }
    }

    pub fn update_physics(&mut self, bodies: &[Body], dt: f32, gm: f32) {
        match self.state {
            ShipState::InTransit { .. } => {
                let r_sq = self.pos.length_squared();
                if r_sq > 0.001 {
                    let r_mag = r_sq.sqrt();
                    let accel = -self.pos * (gm / (r_sq * r_mag));
                    self.vel += accel * dt;
                    self.pos += self.vel * dt;
                }
            }
            ShipState::Docked { body_index }
            | ShipState::Arrived { body_index }
            | ShipState::WaitingForWindow { body_index, .. } => {
                if body_index < bodies.len() {
                    self.pos = bodies[body_index].pos;
                    self.vel = bodies[body_index].vel;
                }
            }
        }
    }

    pub fn update_logic(&mut self, bodies: &[Body], _dt: f32) -> ShipEvent {
        let mut launch_data = None;
        let mut arrive_data = None;

        match &self.state {
            ShipState::WaitingForWindow {
                body_index,
                target_index,
                plan,
            } => {
                let pos1 = bodies[*body_index].pos;
                let pos2 = bodies[*target_index].pos;
                let angle1 = pos1.y.atan2(pos1.x);
                let angle2 = pos2.y.atan2(pos2.x);
                let mut current_phase = angle2 - angle1;
                while current_phase > PI {
                    current_phase -= 2.0 * PI;
                }
                while current_phase <= -PI {
                    current_phase += 2.0 * PI;
                }

                let required = plan.phase_angle_required;
                let diff = (current_phase - required).abs();

                if diff < 0.05 {
                    launch_data = Some((
                        *target_index,
                        plan.clone(),
                        bodies[*body_index].vel,
                        pos1,
                        pos2,
                    ));
                }
            }
            ShipState::InTransit { target_index, .. } => {
                let target = &bodies[*target_index];
                let dist = (self.pos - target.pos).length();
                if dist < target.radius * 2.0 {
                    arrive_data = Some(*target_index);
                }
            }
            _ => {}
        }

        if let Some((target_idx, plan, body_vel, pos1, pos2)) = launch_data {
            let vel_dir = body_vel.normalize_or_zero();
            let r1 = pos1.length();
            let r2 = pos2.length();

            let burn_vec = if r2 > r1 {
                vel_dir * plan.delta_v_departure
            } else {
                -vel_dir * plan.delta_v_departure
            };

            self.vel += burn_vec;
            self.fuel -= plan.delta_v_departure;

            self.state = ShipState::InTransit {
                target_index: target_idx,
                plan,
            };
            return ShipEvent::Launched;
        } else if let Some(target_idx) = arrive_data {
            self.state = ShipState::Arrived {
                body_index: target_idx,
            };
            return ShipEvent::Arrived(target_idx);
        }

        ShipEvent::None
    }
}
