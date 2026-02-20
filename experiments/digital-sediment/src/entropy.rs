use rand::Rng;

pub struct EntropyEngine;

impl EntropyEngine {
    pub fn corrupt(text: &str, decay_factor: f64) -> String {
        if decay_factor <= 0.0 {
            return text.to_string();
        }

        let mut rng = rand::thread_rng();
        let mut corrupted = String::with_capacity(text.len());

        let corruption_chars = [
            '░', '▒', '▓', '█', '?', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+',
            '{', '}', '[', ']', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '.', '/', '`', '~',
        ];

        for c in text.chars() {
            if c.is_whitespace() {
                corrupted.push(c);
                continue;
            }

            let roll: f64 = rng.gen();

            if roll < decay_factor {
                // Apply corruption
                let mutation_type = rng.gen_range(0..10);
                match mutation_type {
                    0..=4 => {
                        // Replace with random glitch char
                        let random_char =
                            corruption_chars[rng.gen_range(0..corruption_chars.len())];
                        corrupted.push(random_char);
                    }
                    5..=7 => {
                        // "Rot" - fade to block
                        let block_char = match decay_factor {
                            d if d > 0.8 => '█',
                            d if d > 0.6 => '▓',
                            d if d > 0.4 => '▒',
                            _ => '░',
                        };
                        corrupted.push(block_char);
                    }
                    8 => {
                        // Bit flip (case swap)
                        if c.is_lowercase() {
                            corrupted.push(c.to_uppercase().next().unwrap_or(c));
                        } else {
                            corrupted.push(c.to_lowercase().next().unwrap_or(c));
                        }
                    }
                    9 => {
                        // Delete (skip) - simulated by not pushing
                        // But to keep alignment somewhat similar, maybe replace with space?
                        corrupted.push(' ');
                    }
                    _ => corrupted.push(c),
                }
            } else {
                corrupted.push(c);
            }
        }
        corrupted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_decay() {
        let text = "Hello World";
        let corrupted = EntropyEngine::corrupt(text, 0.0);
        assert_eq!(text, corrupted);
    }

    #[test]
    fn test_full_decay() {
        let text = "Hello World";
        let corrupted = EntropyEngine::corrupt(text, 1.0);
        assert_ne!(text, corrupted);
        assert_eq!(text.chars().count(), corrupted.chars().count());
    }
}
