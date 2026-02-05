use crate::scanner::Station;
use rand::Rng;

pub struct Tuner {
    pub freq: f64,
    pub velocity: f64,
}

impl Tuner {
    pub fn new() -> Self {
        Self {
            freq: 98.0, // Start in middle
            velocity: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.freq += self.velocity;

        // Friction
        self.velocity *= 0.95;

        // Bounds
        if self.freq < 88.0 {
            self.freq = 88.0;
            self.velocity = 0.0;
        }
        if self.freq > 108.0 {
            self.freq = 108.0;
            self.velocity = 0.0;
        }
    }
}

pub fn demodulate(station: &Station, tuned_freq: f64, bandwidth: f64) -> String {
    let delta = (station.freq - tuned_freq).abs();

    if delta > bandwidth {
        // Pure static
        return generate_static(station.content.len());
    }

    let noise_prob = delta / bandwidth;
    let mut output = String::with_capacity(station.content.len());
    let mut rng = rand::thread_rng();

    // We only process a chunk of the content to avoid lag on huge files during render
    // But for now let's process the whole thing (limit was 100kb)
    for c in station.content.chars() {
        if c.is_whitespace() {
            output.push(c);
        } else if rng.gen::<f64>() < noise_prob {
            output.push(random_char(&mut rng));
        } else {
            output.push(c);
        }
    }

    output
}

fn generate_static(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len.min(1000)).map(|_| random_char(&mut rng)).collect()
}

fn random_char(rng: &mut impl Rng) -> char {
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>/?";
    chars[rng.gen_range(0..chars.len())] as char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_perfect_tuning() {
        let station = Station {
            path: PathBuf::from("test"),
            freq: 98.0,
            content: "Hello World".to_string(),
            size: 11,
        };

        let output = demodulate(&station, 98.0, 0.5);
        assert_eq!(output, "Hello World");
    }

    #[test]
    fn test_bad_tuning() {
        let station = Station {
            path: PathBuf::from("test"),
            freq: 98.0,
            content: "Hello World".to_string(),
            size: 11,
        };

        let output = demodulate(&station, 90.0, 0.5);
        assert_ne!(output, "Hello World");
        // Should be noise (but length might differ due to generate_static clamping)
        // Actually generate_static clamps to 1000, content is 11.
        assert!(output.len() > 0);
    }
}
