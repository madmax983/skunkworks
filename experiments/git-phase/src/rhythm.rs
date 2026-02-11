#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instrument {
    Rest,
    Kick,
    Snare,
    Hat,
}

pub fn generate_pattern(data: &[u8], length: usize) -> Vec<Instrument> {
    if data.is_empty() {
        return vec![Instrument::Rest; length];
    }

    let mut pattern = Vec::with_capacity(length);

    for i in 0..length {
        // Map current step `i` to range in `data`
        let start_idx = (i * data.len()) / length;
        let end_idx = ((i + 1) * data.len()) / length;

        // Ensure at least one byte is read if possible, but respect bounds
        let end_idx = end_idx.max(start_idx + 1).min(data.len());

        if start_idx >= data.len() {
            pattern.push(Instrument::Rest);
            continue;
        }

        let slice = &data[start_idx..end_idx];

        if slice.is_empty() {
             // Fallback if slice is empty (shouldn't happen due to logic above unless data is empty)
             pattern.push(Instrument::Rest);
        } else {
            // Calculate sum of bytes in this chunk
            let sum: u32 = slice.iter().map(|&b| b as u32).sum();

            // Map sum to instrument
            // We use % 5 to make Rests more common (20%) or less common?
            // Prompt says "Rhythm". Rests are important.
            // Let's use % 4: 0=Rest, 1=Kick, 2=Snare, 3=Hat.
            let instrument = match sum % 4 {
                0 => Instrument::Rest,
                1 => Instrument::Kick,
                2 => Instrument::Snare,
                3 => Instrument::Hat,
                _ => unreachable!(),
            };
            pattern.push(instrument);
        }
    }
    pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pattern() {
        let data = vec![1, 2, 3, 4];
        let pattern = generate_pattern(&data, 4);
        assert_eq!(pattern.len(), 4);
        // Logic: chunk size is 1.
        // i=0: data[0..1] sum=1 -> Kick
        // i=1: data[1..2] sum=2 -> Snare
        // i=2: data[2..3] sum=3 -> Hat
        // i=3: data[3..4] sum=4 -> Rest

        assert_eq!(pattern[0], Instrument::Kick);
        assert_eq!(pattern[1], Instrument::Snare);
        assert_eq!(pattern[2], Instrument::Hat);
        assert_eq!(pattern[3], Instrument::Rest);
    }
}
