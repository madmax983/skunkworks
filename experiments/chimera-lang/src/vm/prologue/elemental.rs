use super::normalize_coords;
use crate::vm::prologue::AlchemyRule;
use crate::vm::Value;

pub const ELEM_FIRE: u64 = 0x1F525; // 🔥
pub const ELEM_WATER: u64 = 0x1F4A7; // 💧
pub const ELEM_EARTH: u64 = 0x1F30D; // 🌍
pub const ELEM_AIR: u64 = 0x1F32C; // 🌬️
pub const ELEM_AETHER: u64 = 0x2728; // ✨
pub const ELEM_STEAM: u64 = 0x2601; // ☁️
pub const ELEM_LAVA: u64 = 0x1F30B; // 🌋
pub const ELEM_MUD: u64 = 0x1F331; // 🌱
pub const ELEM_PLASMA: u64 = 0x26A1; // ⚡
pub const ELEM_ASH: u64 = 0x26B0; // ⚰️

pub fn apply_elemental_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    alchemy_book: &[AlchemyRule],
) -> bool {
    let mut changes = false;

    match rune {
        "Δ" => {
            // Fire Source
            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Symbol(ELEM_FIRE));
                changes = true;
            }
        }
        "∇" => {
            // Water Source
            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Symbol(ELEM_WATER));
                changes = true;
            }
        }
        "◊" => {
            // Earth Source
            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Symbol(ELEM_EARTH));
                changes = true;
            }
        }
        "○" => {
            // Air Source
            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Symbol(ELEM_AIR));
                changes = true;
            }
        }
        "☆" => {
            // Aether Source
            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Symbol(ELEM_AETHER));
                changes = true;
            }
        }
        "☿" => {
            // Mercury (Mixer): Reads N, E, S, W. Mixes Elements. Output to Self.
            // If already set (by self or others), don't overwrite unless we have a reaction?
            // "Last Write Wins" applies generally, but here we want to Combine.

            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut ingredients = Vec::new();

            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(val) = &current_signals[ny][nx] {
                        ingredients.push(val.clone());
                    }
                }
            }

            if !ingredients.is_empty() {
                let mut result = None;

                // 1. Check Dynamic Hermetic Rules
                for rule in alchemy_book {
                    // Check if rule.ingredients is a subset of current ingredients
                    // Note: This is a simple subset check. Does not handle counts perfectly (e.g. 2 Fires).
                    // For now, assuming distinct ingredient types or simple presence.
                    let all_match = rule.ingredients.iter().all(|req| ingredients.contains(req));
                    if all_match {
                        result = Some(rule.result.clone());
                        break;
                    }
                }

                // 2. Fallback to Hardcoded Elemental Rules (if no dynamic match)
                if result.is_none() {
                    let mut elements = Vec::new();
                    for val in &ingredients {
                        if let Value::Symbol(id) = val {
                            elements.push(*id);
                        }
                    }

                    if elements.contains(&ELEM_FIRE) && elements.contains(&ELEM_WATER) {
                        result = Some(Value::Symbol(ELEM_STEAM));
                    } else if elements.contains(&ELEM_FIRE) && elements.contains(&ELEM_EARTH) {
                        result = Some(Value::Symbol(ELEM_LAVA));
                    } else if elements.contains(&ELEM_WATER) && elements.contains(&ELEM_EARTH) {
                        result = Some(Value::Symbol(ELEM_MUD));
                    } else if elements.contains(&ELEM_FIRE) && elements.contains(&ELEM_AIR) {
                        result = Some(Value::Symbol(ELEM_PLASMA));
                    } else if elements.contains(&ELEM_FIRE) {
                        result = Some(Value::Symbol(ELEM_FIRE));
                    } else if elements.contains(&ELEM_WATER) {
                        result = Some(Value::Symbol(ELEM_WATER));
                    } else if elements.contains(&ELEM_EARTH) {
                        result = Some(Value::Symbol(ELEM_EARTH));
                    } else if elements.contains(&ELEM_AIR) {
                        result = Some(Value::Symbol(ELEM_AIR));
                    } else if elements.contains(&ELEM_AETHER) {
                        result = Some(Value::Symbol(ELEM_AETHER));
                    }
                }

                if let Some(res_val) = result {
                    if next_signals[y][x] != Some(res_val.clone()) {
                        next_signals[y][x] = Some(res_val);
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}
