use rand::Rng;

pub struct TextGlitcher;

impl TextGlitcher {
    pub fn corrupt(text: &str, intensity: f32) -> String {
        if intensity <= 0.0 {
            return text.to_string();
        }

        let mut rng = rand::thread_rng();
        let mut result = String::with_capacity(text.len());

        for c in text.chars() {
            if rng.gen::<f32>() < intensity {
                // Apply glitch
                let glitch_type = rng.gen_range(0..10);
                match glitch_type {
                    0..=3 => {
                        // Case flip
                        if c.is_uppercase() {
                            result.extend(c.to_lowercase());
                        } else {
                            result.extend(c.to_uppercase());
                        }
                    }
                    4..=6 => {
                        // Leet speak / similar char
                        let replacement = match c {
                            'e' | 'E' => '3',
                            'a' | 'A' => '4',
                            'o' | 'O' => '0',
                            'l' | 'I' => '1',
                            's' | 'S' => '5',
                            't' | 'T' => '7',
                            _ => c,
                        };
                        result.push(replacement);
                    }
                    7..=8 => {
                        // Random ascii
                        result.push(rng.gen_range(33..126) as u8 as char);
                    }
                    9 => {
                        // Zalgo-ish or garbage
                        result.push_str("▒");
                    }
                    _ => result.push(c),
                }
            } else {
                result.push(c);
            }
        }

        // High intensity block corruption
        if intensity > 0.5 && rng.gen::<f32>() < 0.1 {
             result.push_str(" [DATA LOSS] ");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupt_no_intensity() {
        let original = "Hello World";
        let corrupted = TextGlitcher::corrupt(original, 0.0);
        assert_eq!(original, corrupted);
    }

    #[test]
    fn test_corrupt_high_intensity() {
        let original = "Hello World";
        let corrupted = TextGlitcher::corrupt(original, 1.0);
        assert_ne!(original, corrupted);
    }
}
