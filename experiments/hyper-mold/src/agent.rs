use crate::grid::Grid4D;
use crate::math::Vec4;
use rand::Rng;

pub struct Agent {
    pub pos: Vec4,
    pub vel: Vec4,
}

impl Agent {
    pub fn new(pos: Vec4) -> Self {
        let mut rng = rand::thread_rng();
        // Random velocity vector on unit 4-sphere
        let mut vel = Vec4::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        // Avoid zero vector
        if vel.x == 0.0 && vel.y == 0.0 && vel.z == 0.0 && vel.w == 0.0 {
            vel.x = 0.1;
        }

        Self {
            pos,
            vel: vel.normalize(),
        }
    }

    pub fn update(&mut self, grid: &Grid4D, sensor_dist: f32, turn_speed: f32, move_speed: f32) {
        let mut rng = rand::thread_rng();

        // Sensing wrapper with boundary handling
        let sample = |pos: Vec4| -> f32 {
            let width = grid.width as f32;
            let height = grid.height as f32;
            let depth = grid.depth as f32;
            let hypersize = grid.hypersize as f32;

            let mut x = pos.x;
            let mut y = pos.y;
            let mut z = pos.z;
            let mut w = pos.w;

            // Wrap coordinates
            // Simple wrap
            while x < 0.0 {
                x += width;
            }
            while x >= width {
                x -= width;
            }
            while y < 0.0 {
                y += height;
            }
            while y >= height {
                y -= height;
            }
            while z < 0.0 {
                z += depth;
            }
            while z >= depth {
                z -= depth;
            }
            while w < 0.0 {
                w += hypersize;
            }
            while w >= hypersize {
                w -= hypersize;
            }

            grid.get(x as usize, y as usize, z as usize, w as usize)
        };

        // 1. Randomly pick a rotation axis (plane)
        // 4D rotations: XY, XZ, XW, YZ, YW, ZW.
        // Simplified: Pick 3 random orthogonal planes?
        // Let's just perturb the velocity vector directly.

        // Sample Ahead
        let fwd_pos = self.pos.add(self.vel.scale(sensor_dist));
        let c_fwd = sample(fwd_pos);

        // Sample Left/Right equivalent (Random Perturbation)
        // Rotate by a random angle in a random plane
        let angle = rng.gen_range(-0.5..0.5); // Radians
        let plane = rng.gen_range(0..6);

        let trial_vel = match plane {
            0 => self.vel.rotate_xw(angle),
            1 => self.vel.rotate_yw(angle),
            2 => self.vel.rotate_zw(angle),
            // We need 3 more for full coverage: XY, XZ, YZ?
            // Math struct only has XW, YW, ZW implemented for now.
            // Let's stick to XW, YW, ZW as they represent "hyper-turns".
            // To turn in 3D (XY, XZ, YZ), we can just treat it as standard 3D rotation?
            // Let's add simple random vector addition for "general" turn.
            _ => {
                let random_vec = Vec4::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize();
                self.vel.add(random_vec.scale(0.2)).normalize()
            }
        }
        .normalize();

        let trial_pos = self.pos.add(trial_vel.scale(sensor_dist));
        let c_trial = sample(trial_pos);

        // Decision
        if c_trial > c_fwd {
            // Turn towards trial
            // Interpolate velocity
            let mix = turn_speed; // 0.1 to 0.5
            self.vel = self
                .vel
                .scale(1.0 - mix)
                .add(trial_vel.scale(mix))
                .normalize();
        } else {
            // Random wander
            let angle = rng.gen_range(-0.1..0.1);
            let plane = rng.gen_range(0..3);
            self.vel = match plane {
                0 => self.vel.rotate_xw(angle),
                1 => self.vel.rotate_yw(angle),
                _ => self.vel.rotate_zw(angle),
            }
            .normalize();
        }

        // Move
        self.pos = self.pos.add(self.vel.scale(move_speed));

        // Wrap Position
        let width = grid.width as f32;
        let height = grid.height as f32;
        let depth = grid.depth as f32;
        let hypersize = grid.hypersize as f32;

        if self.pos.x < 0.0 {
            self.pos.x += width;
        }
        if self.pos.x >= width {
            self.pos.x -= width;
        }
        if self.pos.y < 0.0 {
            self.pos.y += height;
        }
        if self.pos.y >= height {
            self.pos.y -= height;
        }
        if self.pos.z < 0.0 {
            self.pos.z += depth;
        }
        if self.pos.z >= depth {
            self.pos.z -= depth;
        }
        if self.pos.w < 0.0 {
            self.pos.w += hypersize;
        }
        if self.pos.w >= hypersize {
            self.pos.w -= hypersize;
        }
    }
}
