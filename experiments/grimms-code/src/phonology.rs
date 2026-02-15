pub trait SoundChange {
    fn name(&self) -> &'static str;
    fn apply(&self, input: &str) -> String;
}

pub struct GrimmsLaw;

impl SoundChange for GrimmsLaw {
    fn name(&self) -> &'static str {
        "Grimm's Law (PIE -> Germanic)"
    }

    fn apply(&self, input: &str) -> String {
        let mut result = String::new();
        for c in input.chars() {
            match c {
                'p' => result.push('f'),
                't' => result.push_str("th"), // Simplified t -> th
                'k' => result.push('h'),
                'b' => result.push('p'),
                'd' => result.push('t'),
                'g' => result.push('k'),
                // bh, dh, gh would require digraph checks, let's keep it simple for now
                _ => result.push(c),
            }
        }
        result
    }
}

pub struct HighGermanShift;

impl SoundChange for HighGermanShift {
    fn name(&self) -> &'static str {
        "High German Consonant Shift (Germanic -> OHG)"
    }

    fn apply(&self, input: &str) -> String {
        let chars: Vec<char> = input.chars().collect();
        let mut result = String::new();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let next = if i + 1 < chars.len() {
                Some(chars[i + 1])
            } else {
                None
            };
            let prev = if i > 0 { Some(chars[i - 1]) } else { None };

            // Handle th -> d
            if c == 't' && next == Some('h') {
                result.push('d');
                i += 2;
                continue;
            }

            let is_post_vocalic = prev.map_or(false, is_vowel);
            let is_initial = i == 0 || prev == Some('_');

            match c {
                'p' => {
                    if is_initial {
                        result.push_str("pf");
                    } else if is_post_vocalic {
                        result.push_str("ff");
                    } else {
                        result.push('p');
                    }
                }
                't' => {
                    if is_initial {
                        result.push('z');
                    } else if is_post_vocalic {
                        result.push_str("ss");
                    } else {
                        result.push('z');
                    }
                }
                'k' => {
                    if is_initial {
                        result.push('k');
                    } else if is_post_vocalic {
                        result.push_str("ch");
                    } else {
                        result.push('k');
                    }
                }
                'd' => result.push('t'),
                _ => result.push(c),
            }
            i += 1;
        }
        result
    }
}

pub struct GreatVowelShift;

impl SoundChange for GreatVowelShift {
    fn name(&self) -> &'static str {
        "Great Vowel Shift (MidE -> ModE)"
    }

    fn apply(&self, input: &str) -> String {
        let mut result = String::new();
        for c in input.chars() {
            match c {
                'a' => result.push_str("ei"),
                'e' => result.push('i'),
                'i' => result.push_str("ai"),
                'o' => result.push_str("ou"),
                'u' => result.push_str("au"),
                // Uppercase handling?
                'A' => result.push_str("Ei"),
                'E' => result.push('I'),
                'I' => result.push_str("Ai"),
                'O' => result.push_str("Ou"),
                'U' => result.push_str("Au"),
                _ => result.push(c),
            }
        }
        result
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grimms_law() {
        let law = GrimmsLaw;
        assert_eq!(law.apply("pater"), "father");
        assert_eq!(law.apply("tres"), "thres");
    }

    #[test]
    fn test_high_german() {
        let law = HighGermanShift;
        assert_eq!(law.apply("path"), "pfad"); // p->pf, th->d
        assert_eq!(law.apply("water"), "wasser");
    }
}
