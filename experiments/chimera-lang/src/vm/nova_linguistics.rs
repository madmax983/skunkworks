use super::{ChimeraVM, Value, MAX_COMPLEX_STRING_LEN};

/// Calculates the Levenshtein distance between two strings.
///
/// **OpCode:** `Levenshtein`
/// **Stack:** `[ ..., s1, s2 ] -> [ ..., distance ]`
pub fn exec_levenshtein(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s2_val = vm.stack.pop().unwrap();
        let s1_val = vm.stack.pop().unwrap();

        if let (Value::Str(s1), Value::Str(s2)) = (s1_val, s2_val) {
            // 🔒 WARDEN: DoS Protection
            if s1.len() > MAX_COMPLEX_STRING_LEN || s2.len() > MAX_COMPLEX_STRING_LEN {
                vm.output.push(format!(
                    "Error: String too long for levenshtein (Max {})",
                    MAX_COMPLEX_STRING_LEN
                ));
                // Do not push result
                return None;
            }

            let dist = levenshtein(&s1, &s2);
            vm.stack.push(Value::Int(dist as i64));
            vm.energy = vm.energy.saturating_sub(dist as i64); // Cost proportional to diff
        } else {
            vm.output
                .push("Error: Type mismatch for levenshtein".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for levenshtein".to_string());
    }
    None
}

/// Calculates the Soundex code of a string.
///
/// **OpCode:** `Soundex`
/// **Stack:** `[ ..., string ] -> [ ..., code_string ]`
pub fn exec_soundex(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            if s.len() > MAX_COMPLEX_STRING_LEN {
                vm.output.push(format!(
                    "Error: String too long for soundex (Max {})",
                    MAX_COMPLEX_STRING_LEN
                ));
                return None;
            }
            let code = soundex(&s);
            vm.stack.push(Value::Str(code));
            vm.energy = vm.energy.saturating_sub(5);
        } else {
            vm.output
                .push("Error: Type mismatch for soundex".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for soundex".to_string());
    }
    None
}

/// Checks if two strings are anagrams.
///
/// **OpCode:** `Anagram`
/// **Stack:** `[ ..., s1, s2 ] -> [ ..., is_anagram (1/0) ]`
pub fn exec_anagram(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s2_val = vm.stack.pop().unwrap();
        let s1_val = vm.stack.pop().unwrap();

        if let (Value::Str(s1), Value::Str(s2)) = (s1_val, s2_val) {
            if s1.len() > MAX_COMPLEX_STRING_LEN || s2.len() > MAX_COMPLEX_STRING_LEN {
                vm.output.push(format!(
                    "Error: String too long for anagram (Max {})",
                    MAX_COMPLEX_STRING_LEN
                ));
                return None;
            }
            let is_ana = is_anagram(&s1, &s2);
            vm.stack.push(Value::Int(if is_ana { 1 } else { 0 }));
            vm.energy = vm.energy.saturating_sub(10);
        } else {
            vm.output
                .push("Error: Type mismatch for anagram".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for anagram".to_string());
    }
    None
}

/// Shifts characters in a string by a given amount (Caesar Cipher).
///
/// **OpCode:** `Cipher`
/// **Stack:** `[ ..., shift, string ] -> [ ..., shifted_string ]`
pub fn exec_cipher(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s_val = vm.stack.pop().unwrap();
        let shift_val = vm.stack.pop().unwrap();

        if let (Value::Int(shift), Value::Str(s)) = (shift_val, s_val) {
            // Cipher is linear O(N), so MAX_STRING_LEN (65536) is fine, but sticking to complex limit is safer default
            if s.len() > super::MAX_STRING_LEN {
                vm.output
                    .push("Error: String too long for cipher".to_string());
                return None;
            }
            let shifted = caesar_cipher(&s, shift as i8);
            vm.stack.push(Value::Str(shifted));
            vm.energy = vm.energy.saturating_sub(5);
        } else {
            vm.output
                .push("Error: Type mismatch for cipher".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for cipher".to_string());
    }
    None
}

/// Checks if a string is a pangram (contains all letters a-z).
///
/// **OpCode:** `Pangram`
/// **Stack:** `[ ..., string ] -> [ ..., is_pangram (1/0) ]`
pub fn exec_pangram(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            if s.len() > super::MAX_STRING_LEN {
                vm.output
                    .push("Error: String too long for pangram".to_string());
                return None;
            }
            let is_pan = is_pangram(&s);
            vm.stack.push(Value::Int(if is_pan { 1 } else { 0 }));
            vm.energy = vm.energy.saturating_sub(15);
        } else {
            vm.output
                .push("Error: Type mismatch for pangram".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for pangram".to_string());
    }
    None
}

// --- Helpers ---

fn levenshtein(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let n = s1_chars.len();
    let m = s2_chars.len();

    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

    // Optimization: Use 2 rows to reduce memory from O(N*M) to O(min(N,M))
    let (short, long) = if n < m {
        (&s1_chars, &s2_chars)
    } else {
        (&s2_chars, &s1_chars)
    };

    let min_len = short.len();
    let max_len = long.len();

    let mut prev_row: Vec<usize> = (0..=min_len).collect();
    let mut curr_row: Vec<usize> = vec![0; min_len + 1];

    for i in 1..=max_len {
        curr_row[0] = i;
        for j in 1..=min_len {
            let cost = if long[i - 1] == short[j - 1] { 0 } else { 1 };
            curr_row[j] = std::cmp::min(
                std::cmp::min(curr_row[j - 1] + 1, prev_row[j] + 1),
                prev_row[j - 1] + cost,
            );
        }
        prev_row.clone_from(&curr_row);
    }

    prev_row[min_len]
}

fn soundex(s: &str) -> String {
    let s_upper = s.to_uppercase();
    // Filter out non-alphabetic first
    let chars: Vec<char> = s_upper
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    if chars.is_empty() {
        return "0000".to_string();
    }

    let first = chars[0];

    let map_char = |c: char| -> char {
        match c {
            'B' | 'F' | 'P' | 'V' => '1',
            'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
            'D' | 'T' => '3',
            'L' => '4',
            'M' | 'N' => '5',
            'R' => '6',
            _ => '0', // A, E, I, O, U, H, W, Y
        }
    };

    let mut code = String::with_capacity(4);
    code.push(first);

    let mut last_digit = map_char(first);

    for &c in chars.iter().skip(1) {
        if code.len() >= 4 {
            break;
        }
        let digit = map_char(c);

        if digit != '0' && digit != last_digit {
            code.push(digit);
            last_digit = digit;
        } else if digit == '0' {
            match c {
                'H' | 'W' => {} // Ignore, keep last_digit
                _ => {
                    last_digit = '0';
                } // Reset
            }
        }
    }

    while code.len() < 4 {
        code.push('0');
    }

    code
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut c1: Vec<char> = s1
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    let mut c2: Vec<char> = s2
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    c1.sort();
    c2.sort();
    c1 == c2
}

fn caesar_cipher(s: &str, shift: i8) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = (c as u8 - base) as i16;
                let new_offset = (offset + shift as i16).rem_euclid(26) as u8;
                (base + new_offset) as char
            } else {
                c
            }
        })
        .collect()
}

fn is_pangram(s: &str) -> bool {
    let lower = s.to_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}
