use macroquad::prelude::*;

#[derive(Clone)]
pub struct FluidSim {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<f32>,
    pub water: Vec<f32>,
    // Flux: [Left, Right, Top, Bottom]
    pub flux: Vec<[f32; 4]>,
    pub velocity: Vec<Vec2>,
}

impl FluidSim {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            terrain: vec![0.0; size],
            water: vec![0.0; size],
            flux: vec![[0.0; 4]; size],
            velocity: vec![vec2(0.0, 0.0); size],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        let idx = self.index(x, y);
        self.terrain[idx] + self.water[idx]
    }

    pub fn step(&mut self, dt: f32) {
        let gravity = 9.81;
        let pipe_area = 1.0; // Virtual pipe cross-section
        let damping = 0.99; // Flux damping

        let w = self.width;
        let h = self.height;

        // 1. Update Flux
        // We need a clone or careful indexing to not read partially updated state?
        // Flux depends on current water height. We can update flux in place if we are careful,
        // but typically we need previous state.
        // Let's iterate and update flux in place, but using current heights.
        // Strictly, we should use `flux` from previous step to update water, but here we update flux first based on current water.
        // It's semi-implicit.

        let mut new_flux = self.flux.clone();

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let h1 = self.terrain[idx] + self.water[idx];

                // Left
                if x > 0 {
                    let n_idx = idx - 1;
                    let h2 = self.terrain[n_idx] + self.water[n_idx];
                    let dh = h1 - h2;
                    new_flux[idx][0] += dt * gravity * pipe_area * dh;
                    new_flux[idx][0] *= damping;
                    if new_flux[idx][0] < 0.0 { new_flux[idx][0] = 0.0; }
                } else {
                    new_flux[idx][0] = 0.0; // Boundary
                }

                // Right
                if x < w - 1 {
                    let n_idx = idx + 1;
                    let h2 = self.terrain[n_idx] + self.water[n_idx];
                    let dh = h1 - h2;
                    new_flux[idx][1] += dt * gravity * pipe_area * dh;
                    new_flux[idx][1] *= damping;
                    if new_flux[idx][1] < 0.0 { new_flux[idx][1] = 0.0; }
                } else {
                    new_flux[idx][1] = 0.0;
                }

                // Top
                if y > 0 {
                    let n_idx = idx - w;
                    let h2 = self.terrain[n_idx] + self.water[n_idx];
                    let dh = h1 - h2;
                    new_flux[idx][2] += dt * gravity * pipe_area * dh;
                    new_flux[idx][2] *= damping;
                    if new_flux[idx][2] < 0.0 { new_flux[idx][2] = 0.0; }
                } else {
                    new_flux[idx][2] = 0.0;
                }

                // Bottom
                if y < h - 1 {
                    let n_idx = idx + w;
                    let h2 = self.terrain[n_idx] + self.water[n_idx];
                    let dh = h1 - h2;
                    new_flux[idx][3] += dt * gravity * pipe_area * dh;
                    new_flux[idx][3] *= damping;
                    if new_flux[idx][3] < 0.0 { new_flux[idx][3] = 0.0; }
                } else {
                    new_flux[idx][3] = 0.0;
                }
            }
        }

        // 2. Scaling for Stability
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let sum_out = new_flux[idx][0] + new_flux[idx][1] + new_flux[idx][2] + new_flux[idx][3];
                let max_out = self.water[idx]; // Can't output more than we have

                // If we output more than we have in one step * dt?
                // The flux is a rate (volume/time). So volume = flux * dt.
                // K = min(1.0, water / (sum_out * dt))

                if sum_out * dt > max_out {
                    let k = max_out / (sum_out * dt);
                    new_flux[idx][0] *= k;
                    new_flux[idx][1] *= k;
                    new_flux[idx][2] *= k;
                    new_flux[idx][3] *= k;
                }
            }
        }

        self.flux = new_flux;

        // 3. Update Water Volume
        // dV = dt * (sum(in) - sum(out))
        let mut new_water = self.water.clone();

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;

                let out_vol = dt * (self.flux[idx][0] + self.flux[idx][1] + self.flux[idx][2] + self.flux[idx][3]);

                let mut in_vol = 0.0;
                // In from Left neighbor (their Right flux)
                if x > 0 { in_vol += dt * self.flux[idx - 1][1]; }
                // In from Right neighbor (their Left flux)
                if x < w - 1 { in_vol += dt * self.flux[idx + 1][0]; }
                // In from Top neighbor (their Bottom flux)
                if y > 0 { in_vol += dt * self.flux[idx - w][3]; }
                // In from Bottom neighbor (their Top flux)
                if y < h - 1 { in_vol += dt * self.flux[idx + w][2]; }

                new_water[idx] += in_vol - out_vol;

                // Sanity check
                if new_water[idx] < 0.0 { new_water[idx] = 0.0; }
            }
        }

        self.water = new_water;

        // 4. Update Velocity (approximate)
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                // Average flux passing through cell
                // u = (in_left - out_left + out_right - in_right) / 2
                // Wait, flux is strictly outgoing from cell.
                // Out_Left is flux[idx][0].
                // In_Left is flux[idx-1][1].

                let mut u = 0.0;
                let mut v = 0.0;

                if x > 0 && x < w - 1 {
                     // Simple approx:
                     // u = Out Right - Out Left
                     u = self.flux[idx][1] - self.flux[idx][0];
                }

                if y > 0 && y < h - 1 {
                    v = self.flux[idx][3] - self.flux[idx][2];
                }

                self.velocity[idx] = vec2(u, v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_conservation() {
        let mut sim = FluidSim::new(10, 10);
        sim.water[55] = 10.0; // Drop water in center

        let total_initial: f32 = sim.water.iter().sum();

        sim.step(0.1);

        let total_final: f32 = sim.water.iter().sum();

        assert!((total_initial - total_final).abs() < 1e-4, "Water should be conserved");
    }

    #[test]
    fn test_flow() {
        let mut sim = FluidSim::new(3, 1);
        sim.water[0] = 10.0;
        sim.water[1] = 0.0;
        sim.water[2] = 0.0;

        // Step
        sim.step(0.1);

        assert!(sim.water[0] < 10.0);
        assert!(sim.water[1] > 0.0);
        assert!(sim.water[2] == 0.0); // Hasn't reached yet

        sim.step(0.1);
        assert!(sim.water[2] > 0.0 || sim.water[1] > 0.0);
    }
}
