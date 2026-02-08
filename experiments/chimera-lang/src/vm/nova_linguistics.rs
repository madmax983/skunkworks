#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};

/// Calculates the Levenshtein distance between two strings.
///
/// **OpCode:** `Levenshtein`
/// **Stack:** `[ ..., s1, s2 ] -> [ ..., distance ]`
pub fn exec_levenshtein(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s2_val = vm.stack.pop().unwrap();
        let s1_val = vm.stack.pop().unwrap();

        if let (Value::Str(s1), Value::Str(s2)) = (s1_val, s2_val) {
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

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, val) in dp[0].iter_mut().enumerate() {
        *val = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[n][m]
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
            // For standard soundex, vowels (0) separate consonants, effectively resetting last_digit check
            // BUT ONLY IF it's not H or W.
            // H and W are ignored completely in step 4 check.
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
