use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

/// Applies Phonetic Runes during the sink processing phase.
///
/// Rune:
/// *   `Æ` (Ash) - **Phonetic Transmutation**: Reads West (String). Transmutes to Element (`Δ`, `∇`, `◊`, `○`) at South based on phonetic composition.
pub fn apply_phonetic_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune == "Æ" {
        // Phonetic Transmutation: West (String) -> South (Element)
        if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
            let val = &vm.prologue_state.signal_grid[wy][wx];
            if let Some(Value::Str(s)) = val {
                if !s.is_empty() {
                    let element = analyze_phonetics(s);
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = Value::Str(element.to_string());
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up

                        // Output message for debugging/flavor
                        // vm.output.push(format!("PHONETICS: Transmuted '{}' -> {}", s, element));
                    }
                }
            }
        }
    }
}

/// Analyzes the phonetic composition of a string and returns the dominant Element rune.
///
/// *   **Fire** (`Δ`): Vowels (A, E, I, O, U, Y)
/// *   **Earth** (`◊`): Plosives (B, P, T, D, K, G, C, Q)
/// *   **Air** (`○`): Fricatives (F, V, S, Z, H, J, X)
/// *   **Water** (`∇`): Liquids/Nasals (L, R, M, N, W)
fn analyze_phonetics(s: &str) -> &str {
    let mut fire_score = 0;
    let mut earth_score = 0;
    let mut air_score = 0;
    let mut water_score = 0;

    for c in s.to_lowercase().chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => fire_score += 1,
            'b' | 'p' | 't' | 'd' | 'k' | 'g' | 'c' | 'q' => earth_score += 1,
            'f' | 'v' | 's' | 'z' | 'h' | 'j' | 'x' => air_score += 1,
            'l' | 'r' | 'm' | 'n' | 'w' => water_score += 1,
            _ => {} // Ignore others (numbers, symbols)
        }
    }

    // Determine dominant element
    // Priority: Fire > Earth > Air > Water (arbitrary, but vowels are energetic)
    // Or just max score.

    let max_score = fire_score.max(earth_score).max(air_score).max(water_score);

    if max_score == 0 {
        return "○"; // Default to Air (Void/Empty)
    }

    if fire_score == max_score {
        "Δ"
    } else if earth_score == max_score {
        "◊"
    } else if water_score == max_score {
        "∇"
    } else {
        "○"
    }
}
