use macroquad::prelude::*;
use num_complex::Complex;
use poincare_disk::{mobius_add, Point};
use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Foraging,
    Bridging,
}

#[derive(Clone, Copy)]
pub struct SimParams {
    pub width: usize,
    pub height: usize,
    pub sensor_angle: f64,
    pub sensor_dist: f64,
    pub turn_speed: f64,
    pub move_speed: f64,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Point,
    pub angle: f64,
    pub state: State,
}

impl Agent {
    pub fn new(pos: Point, angle: f64) -> Self {
        Self {
            pos,
            angle,
            state: State::Foraging,
        }
    }

    pub fn update(&mut self, trail_map: &[f32], params: SimParams) {
        if self.state == State::Bridging {
            // Bridges are static structures for now.
            // If the local pheromone density drops too low (bridge abandoned), maybe revert to Foraging?
            // For now, let's keep them stable.
            return;
        }

        // Sensors (reuse from hyperbolic-mold)
        let sensor_l_vec =
            Complex::from_polar(params.sensor_dist, self.angle - params.sensor_angle);
        let sensor_c_vec = Complex::from_polar(params.sensor_dist, self.angle);
        let sensor_r_vec =
            Complex::from_polar(params.sensor_dist, self.angle + params.sensor_angle);

        let sensor_l_pos = mobius_add(self.pos, sensor_l_vec);
        let sensor_c_pos = mobius_add(self.pos, sensor_c_vec);
        let sensor_r_pos = mobius_add(self.pos, sensor_r_vec);

        let val_l = sample_map(trail_map, params.width, params.height, sensor_l_pos);
        let val_c = sample_map(trail_map, params.width, params.height, sensor_c_pos);
        let val_r = sample_map(trail_map, params.width, params.height, sensor_r_pos);

        // Gap Logic
        // Ring gap: 0.4 < r < 0.6
        let r = self.pos.norm();
        let in_gap = r > 0.4 && r < 0.6;

        if in_gap {
            // Check density at current position to decide if we should bridge
            let current_density = sample_map(trail_map, params.width, params.height, self.pos);

            // If density is high enough (meaning other ants are nearby), become a bridge.
            // We need a threshold. Deposit amount is usually ~0.5 per frame.
            // If density > 2.0, likely occupied by a few ants or a persistent trail.
            if current_density > 2.0 {
                self.state = State::Bridging;
                return;
            }
        }

        // Standard movement logic (turn towards pheromones)
        if val_c > val_l && val_c > val_r {
            // Keep straight
        } else if val_c < val_l && val_c < val_r {
            if rand::gen_range(0.0, 1.0) < 0.5 {
                self.angle -= params.turn_speed;
            } else {
                self.angle += params.turn_speed;
            }
        } else if val_l > val_r {
            self.angle -= params.turn_speed;
        } else if val_r > val_l {
            self.angle += params.turn_speed;
        }

        self.angle += rand::gen_range(-0.05, 0.05);

        let move_vec = Complex::from_polar(params.move_speed, self.angle);
        let new_pos = mobius_add(self.pos, move_vec);
        let new_r = new_pos.norm();

        // If trying to enter gap and NOT bridging
        if new_r > 0.4 && new_r < 0.6 {
            // If we are moving into the gap, check if there is a "bridge" (high density) ahead.
            let density_ahead = sample_map(trail_map, params.width, params.height, new_pos);
            let density_current = sample_map(trail_map, params.width, params.height, self.pos);

            // If density ahead is low, it's a void.
            if density_ahead < 0.5 {
                // But if we are crowded at the edge (or on a bridge tip), we can extend the bridge!
                if density_current > 2.0 {
                    // Extend bridge into the void
                    self.pos = new_pos;
                    self.state = State::Bridging;
                    return;
                } else {
                    // Void and not crowded enough to extend. Turn around.
                    self.angle += PI;
                    return;
                }
            }
        }

        // Boundary check
        if new_pos.norm_sqr() < 0.98 {
            self.pos = new_pos;
        } else {
            self.angle += PI;
        }
    }
}

fn sample_map(map: &[f32], width: usize, height: usize, pos: Point) -> f32 {
    if pos.norm_sqr() >= 1.0 {
        return 0.0;
    }

    let x = (pos.re + 1.0) * 0.5 * (width as f64);
    let y = (pos.im + 1.0) * 0.5 * (height as f64);

    let ix = x as usize;
    let iy = y as usize;

    if ix < width && iy < height {
        map[iy * width + ix]
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let pos = Point::new(0.0, 0.0);
        let agent = Agent::new(pos, 0.0);
        assert_eq!(agent.state, State::Foraging);
    }
}
