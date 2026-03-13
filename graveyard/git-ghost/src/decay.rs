use rand::Rng;

const GLITCH_CHARS: &[char] = &[
    '░', '▒', '▓', '█', '▄', '▀', '■', '/', '\\', '?', '#', '$', '%', '&', '@', '!', '^', '*', '(',
    ')', '-', '+', '=', '{', '}', '[', ']', '<', '>', ',', '.', ':', ';', '"', '\'',
];

pub fn apply_decay(text: &str, age_seconds: i64) -> String {
    let mut rng = rand::thread_rng();

    // Decay curve: 1 year = 1.0 entropy (max decay)
    // 1 day = 86400 seconds
    // 1 year = 31536000 seconds
    let max_age = 31536000.0;
    // Map age to 0.0-1.0 range.
    // However, even recent files should have *some* chance of glitch if they are very recent?
    // No, recent files should be clean.
    let entropy = (age_seconds as f64 / max_age).clamp(0.0, 1.0);

    // Make entropy impact more dramatic for visualization purposes
    // Let's say 3 months is enough to be unreadable.
    let effective_entropy = (entropy * 4.0).clamp(0.0, 1.0);

    let mut decayed = String::with_capacity(text.len());

    for line in text.lines() {
        // Chance to drop the line entirely (rot) - only for very old files
        if effective_entropy > 0.8 && rng.gen_bool(effective_entropy * 0.1) {
            continue;
        }

        // Chance to shift the line (melt)
        if effective_entropy > 0.2 && rng.gen_bool(effective_entropy * 0.2) {
            let shift = rng.gen_range(1..5);
            for _ in 0..shift {
                decayed.push(' ');
            }
        }

        for c in line.chars() {
            // Chance to replace char
            // For low entropy, probability is low.
            if rng.gen_bool(effective_entropy * 0.8) {
                if rng.gen_bool(0.3) {
                    // Replace with glitch
                    let glitch = GLITCH_CHARS[rng.gen_range(0..GLITCH_CHARS.len())];
                    decayed.push(glitch);
                } else if rng.gen_bool(0.3) {
                    // Replace with space (fade)
                    decayed.push(' ');
                } else {
                    // Replace with random hex
                    if rng.gen_bool(0.5) {
                        let hex = format!("{:x}", rng.gen_range(0..16));
                        decayed.push_str(&hex);
                    } else {
                        decayed.push('?');
                    }
                }
            } else {
                decayed.push(c);
            }
        }
        decayed.push('\n');
    }

    decayed
}
