use crate::boid::DNA;

pub fn apply_syntax_mutation(dna: &mut DNA, char_eaten: char) {
    if char_eaten.is_ascii_uppercase() {
        // Uppercase: Increase intensity (speed)
        dna.max_speed = (dna.max_speed * 1.1).clamp(0.5, 3.0);
    }

    if char_eaten.is_numeric() {
        // Numbers: Increase order (alignment)
        dna.alignment_weight = (dna.alignment_weight * 1.2).clamp(0.0, 5.0);
    } else if char_eaten.is_ascii_punctuation() || char_eaten.is_whitespace() {
        // Punctuation/Whitespace: Increase chaos (separation), reduce cohesion
        dna.separation_weight = (dna.separation_weight * 1.1).clamp(0.0, 5.0);
        dna.cohesion_weight = (dna.cohesion_weight * 0.9).clamp(0.0, 5.0);
    } else {
        // Letters
        let lower = char_eaten.to_ascii_lowercase();
        match lower {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                // Vowels: Increase openness (view radius)
                dna.view_radius = (dna.view_radius * 1.05).clamp(5.0, 30.0);
            }
            _ => {
                // Consonants: Increase structure (cohesion)
                dna.cohesion_weight = (dna.cohesion_weight * 1.05).clamp(0.0, 5.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    fn mock_dna() -> DNA {
        DNA {
            max_speed: 1.0,
            max_force: 0.1,
            view_radius: 10.0,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            color: Color::White,
            char_representation: '*',
        }
    }

    #[test]
    fn test_vowel_mutation() {
        let mut dna = mock_dna();
        apply_syntax_mutation(&mut dna, 'a');
        assert!(dna.view_radius > 10.0);
    }

    #[test]
    fn test_consonant_mutation() {
        let mut dna = mock_dna();
        apply_syntax_mutation(&mut dna, 'z');
        assert!(dna.cohesion_weight > 1.0);
    }

    #[test]
    fn test_uppercase_mutation() {
        let mut dna = mock_dna();
        apply_syntax_mutation(&mut dna, 'A');
        assert!(dna.max_speed > 1.0);
        // 'A' is also a vowel, so check view radius too
        assert!(dna.view_radius > 10.0);
    }

    #[test]
    fn test_number_mutation() {
        let mut dna = mock_dna();
        apply_syntax_mutation(&mut dna, '1');
        assert!(dna.alignment_weight > 1.0);
    }

    #[test]
    fn test_punctuation_mutation() {
        let mut dna = mock_dna();
        apply_syntax_mutation(&mut dna, '.');
        assert!(dna.separation_weight > 1.0);
        assert!(dna.cohesion_weight < 1.0);
    }
}
