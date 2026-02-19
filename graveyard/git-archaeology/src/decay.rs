use rand::Rng;

/// Corrupts text in-place into the provided output buffer.
/// Does not allocate if the buffer has sufficient capacity.
pub fn corrupt_text_into(text: &str, decay_factor: f32, out: &mut String) {
    out.clear();

    if decay_factor <= 0.0 {
        out.push_str(text);
        return;
    }

    let mut rng = rand::thread_rng();
    let glitch_chars: Vec<char> = "░▒▓█▄▀■□▪▫▲▼◀▶◆◇○●◎★☆☂☁☀⚡❄❅❆".chars().collect();
    // Pre-calculating glitch length (optimization not really needed since we have few chars)

    for c in text.chars() {
        if rng.gen::<f32>() < decay_factor {
            // Replace with a "glitch" character
            let idx = rng.gen_range(0..glitch_chars.len());
            out.push(glitch_chars[idx]);
        } else {
            out.push(c);
        }
    }
}

/// Convenience function if allocation is acceptable (for one-off calls)
pub fn corrupt_text(text: &str, decay_factor: f32) -> String {
    let mut s = String::with_capacity(text.len());
    corrupt_text_into(text, decay_factor, &mut s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupt_text_zero_decay() {
        let text = "Hello World";
        let mut buffer = String::new();
        corrupt_text_into(text, 0.0, &mut buffer);
        assert_eq!(text, buffer);
    }

    #[test]
    fn test_corrupt_text_full_decay() {
        let text = "Hello World";
        let mut buffer = String::new();
        // With 1.0 decay, every char should be replaced.
        corrupt_text_into(text, 1.0, &mut buffer);
        assert_ne!(text, buffer);
        assert_eq!(text.chars().count(), buffer.chars().count());
    }
}
