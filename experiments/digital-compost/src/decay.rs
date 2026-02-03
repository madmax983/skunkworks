use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecayLevel {
    Fresh,
    Stale,
    Moldy,
    Rotting,
    Compost,
}

impl DecayLevel {
    pub fn from_age(age_seconds: i64) -> Self {
        // 1 day = 86400
        // 1 week = 604800
        // 1 month = 2592000
        // 6 months = 15552000
        // 1 year = 31536000
        if age_seconds < 86400 {
            DecayLevel::Fresh
        } else if age_seconds < 604800 {
            DecayLevel::Stale
        } else if age_seconds < 2592000 {
            DecayLevel::Moldy
        } else if age_seconds < 15552000 {
            DecayLevel::Rotting
        } else {
            DecayLevel::Compost
        }
    }

    pub fn color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            DecayLevel::Fresh => Color::Green,
            DecayLevel::Stale => Color::Yellow,
            DecayLevel::Moldy => Color::Rgb(150, 150, 100), // Moldy brownish-green
            DecayLevel::Rotting => Color::Rgb(100, 50, 50), // Rotting brown/red
            DecayLevel::Compost => Color::DarkGray,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            DecayLevel::Fresh => "FRESH",
            DecayLevel::Stale => "STALE",
            DecayLevel::Moldy => "MOLDY",
            DecayLevel::Rotting => "ROTTING",
            DecayLevel::Compost => "COMPOST",
        }
    }
}

pub fn apply_decay(text: &str, level: DecayLevel) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        if c == '\n' {
            result.push(c);
            continue;
        }

        match level {
            DecayLevel::Fresh => result.push(c),
            DecayLevel::Stale => {
                if rng.gen_bool(0.01) {
                    // Slight bit flip or case toggle
                    if c.is_alphabetic() {
                        if c.is_lowercase() {
                            result.push(c.to_ascii_uppercase());
                        } else {
                            result.push(c.to_ascii_lowercase());
                        }
                    } else {
                        // random punctuation change
                        if "!@#$%^&*()".contains(c) {
                            let opts = "!@#$%^&*()";
                            let idx = rng.gen_range(0..opts.len());
                            result.push(opts.chars().nth(idx).unwrap());
                        } else {
                            result.push(c);
                        }
                    }
                } else {
                    result.push(c);
                }
            }
            DecayLevel::Moldy => {
                if rng.gen_bool(0.05) {
                    // Turn into block
                    let blocks = ['░', '▒', '▓'];
                    result.push(blocks[rng.gen_range(0..blocks.len())]);
                } else {
                    result.push(c);
                }
            }
            DecayLevel::Rotting => {
                if rng.gen_bool(0.15) {
                    // Glitch
                    let glitches = [' ', '?', '!', '@', '#', '%', '&', '*', '░', '▒'];
                    result.push(glitches[rng.gen_range(0..glitches.len())]);
                } else if rng.gen_bool(0.05) {
                    // Drop char (skip)
                    // To simulate "hole", push space
                    result.push(' ');
                } else {
                    result.push(c);
                }
            }
            DecayLevel::Compost => {
                if rng.gen_bool(0.50) {
                    let mess = ['░', '▒', '▓', '█', '▄', '▀', '■', ' '];
                    result.push(mess[rng.gen_range(0..mess.len())]);
                } else {
                    // Keep char occasionally to show it was once code
                    result.push(c);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_level_from_age() {
        assert_eq!(DecayLevel::from_age(0), DecayLevel::Fresh);
        assert_eq!(DecayLevel::from_age(100000), DecayLevel::Stale); // > 1 day
        assert_eq!(DecayLevel::from_age(1000000), DecayLevel::Moldy); // > 1 week
        assert_eq!(DecayLevel::from_age(10000000), DecayLevel::Rotting); // > 1 month
        assert_eq!(DecayLevel::from_age(100000000), DecayLevel::Compost); // > 6 months
    }
}
