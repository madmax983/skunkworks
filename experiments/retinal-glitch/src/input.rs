use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum InputPattern {
    MovingBar,
    Noise,
    Circle,
    Gabor,
}

pub struct InputGenerator {
    pub width: usize,
    pub height: usize,
    pub pattern: InputPattern,
    pub time: f32,
}

impl InputGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pattern: InputPattern::MovingBar,
            time: 0.0,
        }
    }

    pub fn next_pattern(&mut self) {
        self.pattern = match self.pattern {
            InputPattern::MovingBar => InputPattern::Circle,
            InputPattern::Circle => InputPattern::Gabor,
            InputPattern::Gabor => InputPattern::Noise,
            InputPattern::Noise => InputPattern::MovingBar,
        };
    }

    pub fn update(&mut self, dt: f32, buffer: &mut [f32]) {
        self.time += dt;
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                if i >= buffer.len() { break; }

                let val = match self.pattern {
                    InputPattern::MovingBar => {
                        // Bar moves right
                        let period = self.width as f32;
                        let bar_x = (self.time * 0.5) % period; // Speed 0.5
                        if (x as f32 - bar_x).abs() < 2.0 {
                            30.0
                        } else {
                            0.0
                        }
                    },
                    InputPattern::Noise => {
                        if rng.gen::<f32>() > 0.90 { 40.0 } else { 0.0 }
                    },
                    InputPattern::Circle => {
                        let cx = self.width as f32 / 2.0;
                        let cy = self.height as f32 / 2.0;
                        let r = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                        // Pulsing radius
                        let radius = 5.0 + (self.time * 0.2).sin().abs() * (self.width as f32 / 4.0);

                        if (r - radius).abs() < 2.0 { 30.0 } else { 0.0 }
                    },
                    InputPattern::Gabor => {
                         // A moving sine wave grating
                         let k = 0.5; // Frequency
                         let v = (x as f32 * k + self.time * 0.2).sin();
                         if v > 0.5 { 30.0 } else { 0.0 }
                    }
                };
                buffer[i] = val;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generation() {
        let mut gen = InputGenerator::new(10, 10);
        let mut buffer = vec![0.0; 100];
        gen.update(1.0, &mut buffer);

        // Moving bar starts near 0.
        // check if some data is non-zero
        assert!(buffer.iter().any(|&x| x > 0.0), "Should generate some input");
    }
}
