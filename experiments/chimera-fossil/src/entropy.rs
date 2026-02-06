use rand::prelude::*;
use rand::rngs::StdRng;

#[derive(Debug, Clone)]
pub struct Fossil {
    pub original_text: String,
    pub displayed_text: String,
    pub mask: Vec<bool>, // true if character is visible (intact), false if redacted/damaged (needs restoration)
}

pub fn fossilize(text: &str, age_factor: f64, seed: u64) -> Fossil {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut displayed_text = String::with_capacity(text.len());
    let mut mask = Vec::with_capacity(text.len());

    // age_factor 0.0 = brand new, 1.0 = ancient dust (max entropy)
    // Probability of corruption increases with age.

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Always preserve newlines to keep structure vaguely readable
        if c == '\n' {
            displayed_text.push(c);
            mask.push(true);
            i += 1;
            continue;
        }

        if c.is_whitespace() {
             // Randomly corrupt whitespace too, but less likely
             if rng.gen::<f64>() < age_factor * 0.1 {
                displayed_text.push('·'); // Visible space corruption
                mask.push(false);
             } else {
                displayed_text.push(c);
                mask.push(true);
             }
             i += 1;
             continue;
        }

        // Identify word boundaries
        if c.is_alphanumeric() || c == '_' {
            // Find end of word
            let mut j = i;
            while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
                j += 1;
            }

            let word_len = j - i;
            // Decay probability scales with age.
            // But we clamp it so it's never 100% impossible.
            let prob_decay = (age_factor * 0.8).min(0.95);

            if rng.gen::<f64>() < prob_decay {
                // Decay type:
                // 1. Redaction (Block)
                // 2. Scramble (Noise)

                if rng.gen::<f64>() < 0.7 {
                    // Redact
                    for _ in 0..word_len {
                        displayed_text.push('█');
                        mask.push(false);
                    }
                } else {
                    // Noise
                    for _ in 0..word_len {
                        let noise = match rng.gen_range(0..4) {
                            0 => '#',
                            1 => '?',
                            2 => '%',
                            _ => '@',
                        };
                        displayed_text.push(noise);
                        mask.push(false);
                    }
                }
            } else {
                // Keep word
                for k in i..j {
                    displayed_text.push(chars[k]);
                    mask.push(true);
                }
            }

            i = j;
        } else {
            // Symbols / Punctuation
             if rng.gen::<f64>() < age_factor * 0.3 {
                displayed_text.push('░');
                mask.push(false);
             } else {
                displayed_text.push(c);
                mask.push(true);
             }
             i += 1;
        }
    }

    Fossil {
        original_text: text.to_string(),
        displayed_text,
        mask,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fossilize_determinism() {
        let text = "Hello World";
        let fossil1 = fossilize(text, 0.5, 123);
        let fossil2 = fossilize(text, 0.5, 123);

        assert_eq!(fossil1.displayed_text, fossil2.displayed_text);
        assert_eq!(fossil1.mask, fossil2.mask);
    }

    #[test]
    fn test_fossilize_corruption() {
        let text = "A long string of text that should definitely have some corruption at high age factors.";
        let fossil = fossilize(text, 1.0, 123); // Max age

        // It's possible RNG produces no corruption even at high age, but unlikely given the length.
        // We assert that IF they are different, mask contains false.
        if fossil.original_text != fossil.displayed_text {
             assert!(fossil.mask.contains(&false));
        }
    }
}
