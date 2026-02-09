use crate::neuron::Izhikevich;

pub struct Retina {
    pub width: usize,
    pub height: usize,
    pub photoreceptors: Vec<f32>,
    pub horizontal: Vec<f32>,
    pub bipolar: Vec<f32>,
    pub ganglion: Vec<Izhikevich>,
}

impl Retina {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            photoreceptors: vec![0.0; size],
            horizontal: vec![0.0; size],
            bipolar: vec![0.0; size],
            ganglion: (0..size).map(|_| Izhikevich::new()).collect(),
        }
    }

    /// Updates the retina state.
    /// Input is expected to be row-major, size width*height.
    /// Returns a list of (x, y) coordinates of spiking ganglion cells.
    pub fn update(&mut self, input: &[f32]) -> Vec<(usize, usize)> {
        if input.len() != self.photoreceptors.len() {
            // Panic or ignore? Panic is safer for debugging mismatch.
            panic!(
                "Input size {} does not match retina size {}",
                input.len(),
                self.photoreceptors.len()
            );
        }

        self.photoreceptors.copy_from_slice(input);

        // 2. Horizontal Cells (Lateral Inhibition)
        // Simple 3x3 box blur for now.
        let w = self.width as i32;
        let h = self.height as i32;

        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum = 0.0;
                let mut count = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < w && ny >= 0 && ny < h {
                            sum += self.photoreceptors[(ny * w + nx) as usize];
                            count += 1.0;
                        }
                    }
                }

                self.horizontal[y * self.width + x] = sum / count;
            }
        }

        // 3. Bipolar Cells (Center-Surround)
        // On-Center: Excitatory Center, Inhibitory Surround.
        // Gain scales the contrast to current.
        let gain = 100.0;

        for i in 0..self.bipolar.len() {
            let center = self.photoreceptors[i];
            let surround = self.horizontal[i];
            // Difference of Gaussians approx
            // P - H > 0 => Center is brighter than average surround
            self.bipolar[i] = (center - surround) * gain;
        }

        // 4. Ganglion Cells (Spiking)
        let mut spikes = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                // Rectify input: Ganglion only fires for positive contrast (On-Center)
                let input_current = self.bipolar[idx].max(0.0);

                if self.ganglion[idx].update(1.0, input_current) {
                    spikes.push((x, y));
                }
            }
        }

        spikes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_interaction() {
        let w = 10;
        let h = 10;
        let mut retina = Retina::new(w, h);
        let mut input = vec![0.0; w * h];

        // Spot of light in the center
        input[5 * w + 5] = 1.0;

        // Run for a few ticks to allow potential to integrate
        let mut spike_count = 0;
        for _ in 0..50 {
            let spikes = retina.update(&input);
            spike_count += spikes.len();
        }

        assert!(
            spike_count > 0,
            "Retina should produce spikes for a spot stimulus"
        );
    }
}
