use crate::dsp::BandPassFilter;
use crate::neuron::{HairCell, LIFNeuron};
use crate::signal::SAMPLE_RATE;

pub struct Channel {
    #[allow(dead_code)]
    pub frequency: f32,
    filter: BandPassFilter,
    hair_cell: HairCell,
    neuron: LIFNeuron,
}

impl Channel {
    pub fn new(freq: f32) -> Self {
        // Q factor increases with frequency in real cochlea, but constant Q is a fine approximation for now.
        // Or Q=4.0
        let q = 4.0;
        let filter = BandPassFilter::new(freq, SAMPLE_RATE, q);
        let hair_cell = HairCell::new(0.9); // Smoothing factor
        let neuron = LIFNeuron::new(0.5, 0.95); // Threshold 0.5, decay 0.95

        Self {
            frequency: freq,
            filter,
            hair_cell,
            neuron,
        }
    }

    pub fn process(&mut self, sample: f32) -> (f32, bool) {
        let filtered = self.filter.process(sample);
        // Gain adjustment: Higher frequencies might need gain boost or reduction.
        // For now, assume flat.
        let potential = self.hair_cell.process(filtered * 5.0); // Boost signal for hair cell
        let spike = self.neuron.process(potential * 5.0); // Drive neuron
        (potential, spike)
    }
}

pub struct Cochlea {
    pub channels: Vec<Channel>,
}

impl Cochlea {
    pub fn new(num_channels: usize) -> Self {
        let mut channels = Vec::with_capacity(num_channels);

        // Logarithmic spacing from 50Hz to 10000Hz
        let min_freq: f32 = 50.0;
        let max_freq: f32 = 10000.0;

        for i in 0..num_channels {
            // Linear in log space
            let t = i as f32 / (num_channels - 1) as f32;
            let freq = min_freq * (max_freq / min_freq).powf(t);
            channels.push(Channel::new(freq));
        }

        Self { channels }
    }

    // Process a chunk of audio
    // Returns:
    // 1. Spikes: List of (time_in_chunk, channel_idx)
    // 2. Potentials: A downsampled heatmap (optional, or full resolution).
    //    For TUI, we probably only need the latest state or average.
    //    Let's return the potentials for the LAST sample in the chunk for visualization.
    pub fn process_chunk(&mut self, samples: &[f32]) -> (Vec<(usize, usize)>, Vec<f32>) {
        let mut spikes = Vec::new();
        let mut last_potentials = vec![0.0; self.channels.len()];

        for (t, &sample) in samples.iter().enumerate() {
            for (ch_idx, channel) in self.channels.iter_mut().enumerate() {
                let (potential, spike) = channel.process(sample);
                if spike {
                    spikes.push((t, ch_idx));
                }
                if t == samples.len() - 1 {
                    last_potentials[ch_idx] = potential;
                }
            }
        }
        (spikes, last_potentials)
    }
}
