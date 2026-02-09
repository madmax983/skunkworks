use macroquad::prelude::*;
use num_complex::Complex;
use poincare_disk::{mobius_add, Point};
use std::f64::consts::PI;

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
}

impl Agent {
    pub fn new(pos: Point, angle: f64) -> Self {
        Self { pos, angle }
    }

    pub fn update(&mut self, trail_map: &[f32], params: SimParams) {
        // Sensors
        let sensor_l_vec = Complex::from_polar(params.sensor_dist, self.angle - params.sensor_angle);
        let sensor_c_vec = Complex::from_polar(params.sensor_dist, self.angle);
        let sensor_r_vec = Complex::from_polar(params.sensor_dist, self.angle + params.sensor_angle);

        let sensor_l_pos = mobius_add(self.pos, sensor_l_vec);
        let sensor_c_pos = mobius_add(self.pos, sensor_c_vec);
        let sensor_r_pos = mobius_add(self.pos, sensor_r_vec);

        let val_l = sample_map(trail_map, params.width, params.height, sensor_l_pos);
        let val_c = sample_map(trail_map, params.width, params.height, sensor_c_pos);
        let val_r = sample_map(trail_map, params.width, params.height, sensor_r_pos);

        // Turn
        if val_c > val_l && val_c > val_r {
            // Keep straight
        } else if val_c < val_l && val_c < val_r {
            // Random turn
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

        // Wobble
        self.angle += rand::gen_range(-0.05, 0.05);

        // Move
        let move_vec = Complex::from_polar(params.move_speed, self.angle);
        let new_pos = mobius_add(self.pos, move_vec);

        // Boundary check (Poincaré disk radius < 1.0)
        if new_pos.norm_sqr() < 0.98 {
            self.pos = new_pos;
        } else {
            self.angle += PI; // Turn around
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
