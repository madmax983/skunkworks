use rand::Rng;

/// A 2D Wave Tank simulation using FDTD method.
pub struct WaveTank {
    pub width: usize,
    pub height: usize,
    current: Vec<f32>,
    previous: Vec<f32>,
    next_buffer: Vec<f32>, // Recycle buffer
    damping: f32,
}

impl WaveTank {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            current: vec![0.0; size],
            previous: vec![0.0; size],
            next_buffer: vec![0.0; size],
            damping: 0.99,
        }
    }

    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;

        // Calculate next state into next_buffer
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;

                let u_c = self.current[idx];
                let u_p = self.previous[idx];

                let u_up = self.current[(y - 1) * w + x];
                let u_down = self.current[(y + 1) * w + x];
                let u_left = self.current[y * w + (x - 1)];
                let u_right = self.current[y * w + (x + 1)];

                let laplacian = u_up + u_down + u_left + u_right - 4.0 * u_c;

                // Wave speed squared
                let c2 = 0.5;

                // Verlet integration
                let val = (u_c * 2.0 - u_p + c2 * laplacian) * self.damping;

                self.next_buffer[idx] = val;
            }
        }

        // Cycle buffers
        // We want: previous <- current, current <- next_buffer, next_buffer <- previous (recycle)

        // 1. Swap previous and current. Now previous has 'current' (t), current has 'previous' (t-1).
        std::mem::swap(&mut self.previous, &mut self.current);

        // 2. Swap current and next_buffer. Now current has 'next_buffer' (t+1), next_buffer has 'previous' (t-1) [which was in current].
        std::mem::swap(&mut self.current, &mut self.next_buffer);

        // Result:
        // current holds t+1
        // previous holds t
        // next_buffer holds t-1 (garbage for next step)
    }

    pub fn disturb(&mut self, x: usize, y: usize, amount: f32) {
        if x > 0 && x < self.width - 1 && y > 0 && y < self.height - 1 {
            let idx = y * self.width + x;
            self.current[idx] += amount;
        }
    }

    pub fn rain(&mut self, drops: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..drops {
            let x = rng.gen_range(1..self.width - 1);
            let y = rng.gen_range(1..self.height - 1);
            self.disturb(x, y, rng.gen_range(0.5..2.0));
        }
    }

    pub fn get_row(&self, y: usize) -> &[f32] {
        let start = y * self.width;
        &self.current[start..start + self.width]
    }

    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.current[y * self.width + x]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disturbance_propagation() {
        let mut tank = WaveTank::new(10, 10);
        tank.disturb(5, 5, 1.0);

        assert_eq!(tank.get_height(5, 5), 1.0);
        assert_eq!(tank.get_height(6, 5), 0.0);

        tank.step();

        // After 1 step, it might spread immediately depending on scheme or next step
        // With Verlet, current becomes previous, next is calc.
        // t=0: current has spike. previous has 0.
        // t=1: next = 2*current - prev + c2*laplacian
        // center: 2*1 - 0 + 0.5 * (0+0+0+0 - 4) = 2 - 2 = 0?
        // neighbors: 2*0 - 0 + 0.5 * (1+0+0+0 - 0) = 0.5.
        // So center goes to 0, neighbors go to 0.5.

        assert!(
            tank.get_height(6, 5) > 0.0,
            "Wave should propagate to neighbor"
        );
        assert!(
            tank.get_height(4, 5) > 0.0,
            "Wave should propagate to neighbor"
        );
        assert!(
            tank.get_height(5, 6) > 0.0,
            "Wave should propagate to neighbor"
        );
        assert!(
            tank.get_height(5, 4) > 0.0,
            "Wave should propagate to neighbor"
        );
    }
}
