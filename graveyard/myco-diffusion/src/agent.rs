use gray_scott::GrayScott;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AgentState {
    CommutingToWork,
    CommutingHome,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
    pub home: Vec2,
    pub work: Vec2,
    pub state: AgentState,
}

impl Agent {
    pub fn new(pos: Vec2, angle: f32, home: Vec2, work: Vec2) -> Self {
        Self {
            pos,
            angle,
            home,
            work,
            state: AgentState::CommutingToWork,
        }
    }

    pub fn update(&mut self, grid: &GrayScott, settings: &Settings) {
        let sensor_angle = settings.sensor_angle;
        let sensor_dist = settings.sensor_dist;
        let turn_angle = settings.turn_angle;

        // Sensing (Physarum Logic) - Sense 'V' concentration (Active Chemical)
        let sense = |angle_offset: f32| -> f32 {
            let angle = self.angle + angle_offset;
            let dir = Vec2::new(angle.cos(), angle.sin());
            let sensor_pos = self.pos + dir * sensor_dist;

            // Wrap coordinates
            let w = grid.width() as f32;
            let h = grid.height() as f32;
            let x = (sensor_pos.x.rem_euclid(w)) as usize;
            let y = (sensor_pos.y.rem_euclid(h)) as usize;

            if x < grid.width() && y < grid.height() {
                grid.v()[y * grid.width() + x] // Read V
            } else {
                0.0
            }
        };

        let left = sense(-sensor_angle);
        let center = sense(0.0);
        let right = sense(sensor_angle);

        // Turn based on sensors
        let mut turned = false;
        if center > left && center > right {
            // Stay straight
        } else if center < left && center < right {
            // Randomly turn left or right
            let mut rng = ::rand::thread_rng();
            use ::rand::Rng;
            if rng.gen_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
            turned = true;
        } else if left > right {
            self.angle -= turn_angle;
            turned = true;
        } else if right > left {
            self.angle += turn_angle;
            turned = true;
        }

        // Target Bias (Commuter Logic)
        let target = match self.state {
            AgentState::CommutingToWork => self.work,
            AgentState::CommutingHome => self.home,
        };

        // If close to target, switch state
        if self.pos.distance(target) < 10.0 {
            self.state = match self.state {
                AgentState::CommutingToWork => AgentState::CommutingHome,
                AgentState::CommutingHome => AgentState::CommutingToWork,
            };
            self.angle += std::f32::consts::PI; // Turn around
        }

        // Apply bias towards target
        // If we just turned based on trail, apply less bias to respect the trail
        let bias_strength = if turned { 0.05 } else { 0.1 };

        let to_target = target - self.pos;
        let target_angle = to_target.y.atan2(to_target.x);
        let angle_diff = (target_angle - self.angle + std::f32::consts::PI)
            .rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;

        // Nudge
        self.angle += angle_diff * bias_strength;

        // Move
        let dir = Vec2::new(self.angle.cos(), self.angle.sin());
        self.pos += dir * settings.move_speed;

        // Wrap Position
        self.pos.x = self.pos.x.rem_euclid(grid.width() as f32);
        self.pos.y = self.pos.y.rem_euclid(grid.height() as f32);
    }
}

pub struct Settings {
    pub sensor_angle: f32,
    pub sensor_dist: f32,
    pub turn_angle: f32,
    pub move_speed: f32,
    pub deposit_amount: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sensor_angle: std::f32::consts::PI / 4.0,
            sensor_dist: 9.0,
            turn_angle: std::f32::consts::PI / 4.0,
            move_speed: 1.0,
            deposit_amount: 0.1, // Feed the reaction
        }
    }
}
