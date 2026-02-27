use neuro_sim::Izhikevich;

pub struct Retina {
    pub width: usize,
    pub height: usize,
    pub neurons: Vec<Izhikevich>,
    pub inputs: Vec<f32>,
    pub spikes: Vec<bool>,
}

impl Retina {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let neurons = vec![Izhikevich::new(); size];
        let inputs = vec![0.0; size];
        let spikes = vec![false; size];

        Self {
            width,
            height,
            neurons,
            inputs,
            spikes,
        }
    }

    pub fn set_input(&mut self, x: usize, y: usize, ch: char) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;

        // Convert char to current intensity
        let intensity = match ch {
            ' ' | '\t' | '\n' => 0.0,
            // High salience characters (Brackets, Operators)
            '{' | '}' | '(' | ')' | '[' | ']' | '<' | '>' => 25.0,
            '+' | '-' | '*' | '/' | '=' | '&' | '|' | '!' | ':' | ';' => 20.0,
            // Keywords often start with low case
            'a'..='z' => 5.0,
            // Types often start with Upper case
            'A'..='Z' => 10.0,
            // Numbers
            '0'..='9' => 8.0,
            _ => 2.0,
        };

        self.inputs[idx] = intensity;
    }

    pub fn update(&mut self, dt: f32) {
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let input = self.inputs[i];
            let (_, spiked) = neuron.update(dt, input);
            self.spikes[i] = spiked;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retina_spike() {
        let mut retina = Retina::new(10, 10);
        // Inject high current at (5,5)
        retina.set_input(5, 5, '{');

        // Run for enough steps to spike
        let mut spiked = false;
        for _ in 0..100 {
            retina.update(1.0);
            if retina.spikes[5 * 10 + 5] {
                spiked = true;
                break;
            }
        }
        assert!(
            spiked,
            "Neuron at (5,5) should have spiked given '{{' input"
        );
    }
}
