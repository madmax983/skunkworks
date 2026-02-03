use crate::neuron::Neuron;

pub struct Retina {
    pub width: usize,
    pub height: usize,
    pub neurons: Vec<Neuron>,
    pub inhibition_weight: f32,
    pub last_spikes: Vec<bool>,
}

impl Retina {
    pub fn new(width: usize, height: usize) -> Self {
        let count = width * height;
        let neurons = vec![Neuron::regular_spiking(); count];
        let last_spikes = vec![false; count];

        Self {
            width,
            height,
            neurons,
            inhibition_weight: 10.0, // Significant inhibition
            last_spikes,
        }
    }

    pub fn update(&mut self, input: &[f32]) {
        let mut new_spikes = vec![false; self.neurons.len()];

        for i in 0..self.neurons.len() {
            let y = i / self.width;
            let x = i % self.width;

            let mut inhibition = 0.0;

            // Check neighbors (Left, Right, Up, Down)
            let neighbors = [
                if x > 0 { Some(i - 1) } else { None }, // Left
                if x < self.width - 1 {
                    Some(i + 1)
                } else {
                    None
                }, // Right
                if y > 0 { Some(i - self.width) } else { None }, // Up
                if y < self.height - 1 {
                    Some(i + self.width)
                } else {
                    None
                }, // Down
            ];

            for n_idx in neighbors.iter().flatten() {
                if self.last_spikes[*n_idx] {
                    inhibition += self.inhibition_weight;
                }
            }

            let current = input[i] - inhibition;
            self.neurons[i].update(current, 1.0);

            new_spikes[i] = self.neurons[i].spiked;
        }

        self.last_spikes = new_spikes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lateral_inhibition() {
        let mut retina = Retina::new(2, 1);
        retina.inhibition_weight = 50.0; // Strong inhibition

        // Control: Run only neuron 1 (index 1) with moderate input
        let mut control_spikes = 0;
        let mut n = Neuron::regular_spiking();
        for _ in 0..100 {
            n.update(10.0, 1.0);
            if n.spiked {
                control_spikes += 1;
            }
        }

        // Experiment: Run neuron 0 (high input) and neuron 1 (moderate input)
        // Neuron 0 should spike and inhibit Neuron 1
        let mut experiment_spikes = 0;

        // We need to run the retina update loop
        // Input: [High, Moderate]
        let input = vec![100.0, 10.0];

        for _ in 0..100 {
            retina.update(&input);
            if retina.neurons[1].spiked {
                experiment_spikes += 1;
            }
        }

        println!(
            "Control: {}, Experiment: {}",
            control_spikes, experiment_spikes
        );
        assert!(
            experiment_spikes < control_spikes,
            "Lateral inhibition should reduce spike count of neighbor"
        );
    }
}
