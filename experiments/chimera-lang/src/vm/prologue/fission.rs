//! # Fission Runes ☢️
//!
//! The Fission module introduces **Radioactive Decay** and **Particle Physics** to the Prologue grid.
//! It simulates nuclear reactors, control rods, and neutron moderators.
//!
//! ## Philosophy
//!
//! Fission is about **Entropy** and **Decay**. Signals degrade as they propagate,
//! splitting into smaller components or being absorbed entirely.
//!
//! ## Rune Reference
//!
//! | Rune | Name | Input | Output | Description |
//! |---|---|---|---|---|
//! | `✦` | **Neutron Source** | None | Self | Emits random radiation (Int). |
//! | `☢` | **Reactor Core** | West (Energy) | North, East, South | Splits energy: `Val/2` to N/E/S. |
//! | `✇` | **Control Rod** | West (Energy) | None | Absorbs all radiation (Sink). |
//! | `⌘` | **Moderator** | West (Energy) | East | Slows/Cools radiation: `Val/2` to East. |

use super::normalize_coords;
use crate::vm::Value;
use rand::Rng;

/// Prepares initial signals for Fission Sources (`✦`).
///
/// This is called once per tick during the Signal phase.
pub fn prepare_fission_sources(
    rune: &str,
    y: usize,
    x: usize,
    signal_grid: &mut Vec<Vec<Option<Value>>>,
) {
    if rune == "✦" {
        // Emit random radiation
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(50..100);
        signal_grid[y][x] = Some(Value::Int(val));
    }
}

/// Applies propagation logic for Fission Runes (`☢`, `✇`, `⌘`).
pub fn apply_fission_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;

    // Get input from West
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "☢" => {
            // Reactor Core: Splits input energy
            if let Some(Value::Int(n)) = w_sig {
                if n > 1 {
                    let half = n / 2;
                    let output = Value::Int(half);

                    // Light up self (so East neighbor can read as source)
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(output.clone());
                        changes = true;
                    }

                    // Output to North
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(output.clone());
                            changes = true;
                        }
                    }
                    // Output to East
                    if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        if next_signals[ey][ex].is_none() {
                            next_signals[ey][ex] = Some(output.clone());
                            changes = true;
                        }
                    }
                    // Output to South
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(output.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "✇" => {
            // Control Rod: Absorbs signal (Do nothing, effectively a Sink for propagation)
            // But we might want to light up self to show absorption
            if w_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(0)); // Absorbed/Inert
                    changes = true;
                }
            }
        }
        "⌘" => {
            // Moderator: Slows signal (Halves it) and passes East
            if let Some(Value::Int(n)) = w_sig {
                let moderated = Value::Int(n / 2);

                // Output to East
                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(moderated);
                        changes = true;
                    }
                }

                // Light up self
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(n));
                    changes = true;
                }
            }
        }
        _ => {}
    }

    changes
}
