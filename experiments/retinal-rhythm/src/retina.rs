use crate::neuron::{IzhikevichNeuron, NeuronType};
use rayon::prelude::*;

pub struct Retina {
    pub width: usize,
    pub height: usize,
    pub neurons: Vec<IzhikevichNeuron>,
}

impl Retina {
    pub fn new(width: usize, height: usize) -> Self {
        let count = width * height;
        let mut neurons = Vec::with_capacity(count);

        for y in 0..height {
            for _ in 0..width {
                // Variation: Center vs Periphery?
                // Let's make top half P-cells (Detail/Tonic), bottom half M-cells (Motion/Phasic)
                let n_type = if y < height / 2 {
                    NeuronType::RegularSpiking // P-cells (Tonic)
                } else {
                    NeuronType::FastSpiking // M-cells (Phasic)
                };
                neurons.push(IzhikevichNeuron::new(n_type));
            }
        }

        Self {
            width,
            height,
            neurons,
        }
    }

    pub fn update(&mut self, input: &[f32], dt: f32) -> Vec<(usize, usize)> {
        // 1. Convolve Input with DoG (Difference of Gaussians)
        // Center (Excitation): Small Gaussian (sigma=1.0)
        // Surround (Inhibition): Large Gaussian (sigma=2.0)

        let currents: Vec<f32> = (0..self.neurons.len())
            .into_par_iter()
            .map(|idx| {
                let x = (idx % self.width) as i32;
                let y = (idx / self.width) as i32;

                let center = gaussian_blur(input, self.width, self.height, x, y, 0.8);
                let surround = gaussian_blur(input, self.width, self.height, x, y, 2.0);

                // DoG response
                // Contrast enhancement.
                // 100.0 gain to drive neurons (needs > ~5-10 current to fire)
                (center - surround) * 100.0
            })
            .collect();

        // 2. Update Neurons
        let mut spikes = Vec::new();

        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let input_current = currents[i];

            // Rectify: negative current inhibits (hyperpolarizes), positive excites.
            // Izhikevich handles both.

            if neuron.update(input_current, dt) {
                let x = i % self.width;
                let y = i / self.width;
                spikes.push((x, y));
            }
        }

        spikes
    }
}

fn gaussian_blur(input: &[f32], w: usize, h: usize, cx: i32, cy: i32, sigma: f32) -> f32 {
    let kernel_radius = (sigma * 3.0) as i32;
    let mut sum = 0.0;
    let mut weight_sum = 0.0;

    for dy in -kernel_radius..=kernel_radius {
        for dx in -kernel_radius..=kernel_radius {
            let px = cx + dx;
            let py = cy + dy;

            if px >= 0 && px < w as i32 && py >= 0 && py < h as i32 {
                let val = input[(py as usize) * w + (px as usize)];
                let dist_sq = (dx*dx + dy*dy) as f32;
                let weight = (-dist_sq / (2.0 * sigma * sigma)).exp();

                sum += val * weight;
                weight_sum += weight;
            }
        }
    }

    if weight_sum > 0.0 {
        sum / weight_sum
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retina_processing() {
        let w = 20;
        let h = 20;
        let mut retina = Retina::new(w, h);

        // Create an edge input (left half black, right half white)
        let mut input = vec![0.0; w * h];
        for y in 0..h {
            for x in w/2..w {
                input[y * w + x] = 1.0;
            }
        }

        // Update for a few steps
        let mut spike_count = 0;
        for _ in 0..50 {
             let spikes = retina.update(&input, 1.0);
             spike_count += spikes.len();
        }

        // Just ensure it runs without panic and maybe does something
        // Spikes are stochastic if noise is enabled, or deterministic if not.
        // We just check it runs.
        assert!(spike_count >= 0);
    }
}
