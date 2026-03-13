use num_complex::Complex;
use rustfft::FftPlanner;
use std::f64::consts::PI;

use crate::simulation::{World, WORLD_SIZE};

pub struct Hologram {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Complex<f64>>,
}

impl Hologram {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Complex::new(0.0, 0.0); width * height],
        }
    }

    pub fn from_world(world: &World, width: usize, height: usize) -> Self {
        let mut grid = vec![0.0; width * height];

        let scale_x = width as f64 / WORLD_SIZE;
        let scale_y = height as f64 / WORLD_SIZE;

        // Populate density grid from agent positions
        for agent in &world.agents {
            let cx = (agent.pos.0 * scale_x).round() as isize;
            let cy = (agent.pos.1 * scale_y).round() as isize;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                        // Center is brighter
                        let w = if dx == 0 && dy == 0 { 1.0 } else { 0.5 };
                        grid[(py as usize) * width + (px as usize)] += w;
                    }
                }
            }
        }

        // Add Target
        let tx = (world.target.0 * scale_x).round() as isize;
        let ty = (world.target.1 * scale_y).round() as isize;
        for dy in -3..=3 {
            for dx in -3..=3 {
                let px = tx + dx;
                let py = ty + dy;
                if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                    grid[(py as usize) * width + (px as usize)] += 5.0;
                }
            }
        }

        // Apply Reference Beam (Recording Angle)
        let rec_angle_x = 20.0 * (2.0 * PI / width as f64); // Shift by 20 bins
        let rec_angle_y = 10.0 * (2.0 * PI / height as f64); // Shift by 10 bins

        let mut data: Vec<Complex<f64>> = grid
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = (i % width) as f64;
                let y = (i / width) as f64;
                // Modulation: exp(i * (kx*x + ky*y))
                let phase = rec_angle_x * x + rec_angle_y * y;
                let r = Complex::new(phase.cos(), phase.sin());
                Complex::new(val, 0.0) * r
            })
            .collect();

        // 2D FFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_forward(width);
        let fft_y = planner.plan_fft_forward(height);

        // FFT Rows
        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut data[start..end]);
        }

        // Transpose
        transpose(&mut data, width, height);

        // FFT Columns (which are now rows in memory)
        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut data[start..end]);
        }

        // Transpose back
        transpose(&mut data, height, width);

        Self {
            width,
            height,
            data,
        }
    }

    pub fn reconstruct(&self, angle_x: isize, angle_y: isize) -> Vec<f64> {
        let mut planner = FftPlanner::new();
        let ifft_x = planner.plan_fft_inverse(self.width);
        let ifft_y = planner.plan_fft_inverse(self.height);

        let mut recon_data = self.data.clone();

        // 2D IFFT
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;
            ifft_x.process(&mut recon_data[start..end]);
        }

        transpose(&mut recon_data, self.width, self.height);

        for x in 0..self.width {
            let start = x * self.height;
            let end = start + self.height;
            ifft_y.process(&mut recon_data[start..end]);
        }

        transpose(&mut recon_data, self.height, self.width);

        let shift_x = angle_x as f64 * (2.0 * PI / self.width as f64);
        let shift_y = angle_y as f64 * (2.0 * PI / self.height as f64);

        recon_data
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let x = (i % self.width) as f64;
                let y = (i / self.width) as f64;
                let phase = shift_x * x + shift_y * y;
                let r = Complex::new(phase.cos(), phase.sin());
                // Demodulate
                (c * r).norm()
            })
            .collect()
    }
}

fn transpose<T: Clone>(data: &mut [T], width: usize, height: usize) {
    let mut temp = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            temp.push(data[y * width + x].clone());
        }
    }
    data.clone_from_slice(&temp);
}
